use std::{collections::HashMap, error::Error, io::Cursor, net::SocketAddr, sync::Arc};

use deku::{DekuWriter, writer::Writer};
use pretty_hex::pretty_hex;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
    runtime::Handle,
    sync::Mutex,
    task::{JoinHandle, LocalSet},
};

use crate::{
    commands::{Command, shared::horse::Horse},
    database::Database,
    entities::{account::Account, character::Character},
    handlers::{
        PacketHandler,
        lobby::{
            achievement_complete_list::AchievementCompleteListHandler,
            create_nickname::CreateNicknameHandler, get_messenger_info::GetMessengerInfoHandler,
            login::LoginHandler, request_daily_quest_list::RequestDailyQuestListHandler,
            request_league_info::RequestLeagueInfoHandler,
            request_quest_list::RequestQuestListHandler,
            request_special_event_list::RequestSpecialEventListHandler,
            show_inventory::ShowInventoryHandler,
        },
        ranch::{
            breeding_failure_card::BreedingFailureCardHandler,
            breeding_wishlist::BreedingWishlistHandler,
            enter_breeding_market::EnterBreedingMarketHandler,
            leave_breeding_market::LeaveBreedingMarketHandler,
            mount_family_tree::MountFamilyTreeHandler, ranch_chat::RanchChatHandler,
            ranch_cmd_action::RanchCmdActionHandler, ranch_snapshot::RanchSnapshotHandler,
            request_npc_dress_list::RequestNpcDressListHandler,
            request_storage::RequestStorageHandler, search_stallion::SearchStallionHandler,
            try_breeding::TryBreedingHandler, update_mount_nickname::UpdateMountNicknameHandler,
            wear_equipment::WearEquipmentHandler,
        },
    },
    packet::{CommandId, MAX_BUFFER_SIZE, Packet, PacketScrambler},
    ranch::Ranch,
    settings::Settings,
};

pub struct Session {
    buf: [u8; MAX_BUFFER_SIZE],
    socket: TcpStream,

    pub scrambler: PacketScrambler,

    pub account: Option<Account>,
    pub character: Option<Character>,
    pub horses: Option<Vec<Horse>>,

    pub ranch_id: Option<u32>,
}
impl Session {
    fn new(socket: TcpStream) -> Self {
        Session {
            buf: [0u8; MAX_BUFFER_SIZE],
            socket,

            scrambler: PacketScrambler::default(),

            account: None,
            character: None,
            horses: None,

            ranch_id: None,
        }
    }

    pub fn get_mount(&self) -> Option<&Horse> {
        if self.horses.is_none() || self.character.is_none() {
            None
        } else {
            self.horses
                .as_ref()
                .unwrap()
                .iter()
                .filter(|h| h.uid == self.character.as_ref().unwrap().mount_uid)
                .next()
        }
    }

    pub async fn recv_packet(&mut self) -> Result<Packet, String> {
        let mut packet = Packet::from_stream(&mut self.buf, &mut self.socket)
            .await
            .map_err(|e| format!("Error reading command: {}:\n\t{}", e, pretty_hex(&self.buf)))?;
        self.scrambler.scramble(&mut packet);
        Ok(packet)
    }

    pub async fn send_command<T>(&mut self, command: T) -> Result<(), String>
    where
        T: Command,
    {
        let formatted_command = format!("{:#?}", command);
        let packet = command
            .try_into()
            .map_err(|e| format!("Failed to serialize response: {:?}", e))?;
        self.do_send_packet(&packet).await?;
        if !packet.command_id.muted() {
            println!(
                ">>> Sent command {:?}:\n\tLength: {} ({:#x}) bytes\n{}\n\n",
                T::ID,
                packet.payload.len(),
                packet.payload.len(),
                formatted_command
            );
        }
        Ok(())
    }

    pub async fn send_packet(&mut self, packet: &Packet) -> Result<(), String> {
        self.do_send_packet(packet).await?;
        if !packet.command_id.muted() {
            println!(
                ">>> Sent packet {:?}:\n\t{}\n\n",
                packet.command_id,
                pretty_hex(&packet.payload)
            );
        }
        Ok(())
    }

    async fn do_send_packet(&mut self, packet: &Packet) -> Result<(), String> {
        // Outgoing commands aren't scrambled, so we can write directly to the buffer
        let written_bytes = {
            let mut cursor = Cursor::new(&mut self.buf[..]);
            let mut writer = Writer::new(&mut cursor);
            packet
                .to_writer(&mut writer, ())
                .map_err(|err| format!("Error serializing command: {}", err))?;
            writer.bits_written / 8
        };
        let written_bytes = &self.buf[0..written_bytes];
        self.socket
            .write(&written_bytes)
            .await
            .map_err(|err| format!("Error sending command: {:?}", err))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ServerType {
    Lobby,
    Ranch,
}

pub struct Server {
    pub server_type: ServerType,
    pub settings: Settings,
    pub database: Arc<Mutex<Database>>,

    pub sessions: HashMap<SocketAddr, Arc<Mutex<Session>>>,
    pub ranches: HashMap<u32, Ranch>,

    worker_task: Option<JoinHandle<()>>,
    stop: bool,
}
impl Server {
    pub async fn new(
        server_type: ServerType,
        settings: &Settings,
        database: Arc<Mutex<Database>>,
    ) -> Result<Arc<Mutex<Server>>, Box<dyn Error>> {
        let bind_address = match server_type {
            ServerType::Lobby => &settings.lobby_server.bind_address,
            ServerType::Ranch => &settings.ranch_server.bind_address,
        };

        let tcp_listener = TcpListener::bind(bind_address).await?;
        println!("{:?} server listening on: {}", server_type, bind_address);

        let server_instance = Arc::new(Mutex::new(Server {
            server_type: server_type,
            settings: settings.clone(),
            database: Arc::clone(&database),

            sessions: HashMap::new(),
            ranches: HashMap::new(),

            worker_task: None,
            stop: false,
        }));

        // Spawn a task to deal with all incoming connections
        let server = Arc::clone(&server_instance);
        let worker_task = tokio::spawn(async move {
            while !server.lock().await.stop {
                // Asynchronously wait for an inbound socket.
                let accept_result = tcp_listener.accept().await;
                if let Err(e) = accept_result {
                    eprintln!("Failed to accept socket: {}", e);
                    return;
                }

                let (socket, _) = accept_result.unwrap();
                // Use peer address as identifier when storing sessions in the hash map
                let peer_addr = socket.peer_addr();
                if let Err(e) = peer_addr {
                    eprintln!("Failed to obtain peer address: {}", e);
                    return;
                }
                let peer_addr = peer_addr.unwrap();

                // Spawn a task for each accepted connection
                let server = Arc::clone(&server);
                tokio::task::spawn_blocking(move || {
                    Handle::current().block_on(async move {
                        println!("New connection established");
                        let session = Arc::new(Mutex::new(Session::new(socket)));
                        server.lock().await.sessions.insert(peer_addr, Arc::clone(&session));

                        let server = Arc::clone(&server);
                        // In a loop, handle incoming data until the server is stopped or we break the loop.
                        while !server.lock().await.stop {
                            // Local task set for every task spawned while handling this packet.
                            // Tasks in this set will all run on the same thread, allowing to run code that cant
                            // be moved across threads, such as postgres transactions
                            // TODO: Look into FuturesUnordered
                            let local_task_set = LocalSet::new();
                            // Spawn a task for each incoming packet. These tasks will not be run in parallel
                            // but we need them as tasks running in the LocalSet to ensure all the child tasks
                            // are run in the same thread.
                            let session = Arc::clone(&session);
                            let server = Arc::clone(&server);
                            let handle_result = local_task_set
                                .run_until(async move {
                                    tokio::task::spawn_local(async move {
                                        // Receive the next packet
                                        let packet = session.lock().await.recv_packet().await.map_err(|err| {
                                                format!("Failed to receive packet: {}", err)
                                            })?;

                                        let handle_result = match server_type {
                                            // Lobby server commands
                                            ServerType::Lobby => match packet.command_id {
                                                CommandId::AcCmdCLAchievementCompleteList => {
                                                    AchievementCompleteListHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCLCreateNickname => {
                                                    CreateNicknameHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCLEnterRanch => {
                                                    crate::handlers::lobby::enter_ranch::EnterRanchHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCLGetMessengerInfo => {
                                                    GetMessengerInfoHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCLLogin => {
                                                    LoginHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCLRequestDailyQuestList => {
                                                    RequestDailyQuestListHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCLRequestLeagueInfo => {
                                                    RequestLeagueInfoHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCLRequestQuestList => {
                                                    RequestQuestListHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCLRequestSpecialEventList => {
                                                    RequestSpecialEventListHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCLShowInventory => {
                                                    ShowInventoryHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                _ => Err("Unhandled command".into()),
                                            },
                                            // Ranch server commands
                                            ServerType::Ranch => match packet.command_id {
                                                CommandId::AcCmdCRBreedingFailureCard => {
                                                    BreedingFailureCardHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCRBreedingWishlist => {
                                                    BreedingWishlistHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCREnterBreedingMarket => {
                                                    EnterBreedingMarketHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCREnterRanch => {
                                                    crate::handlers::ranch::enter_ranch::EnterRanchHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCRLeaveBreedingMarket => {
                                                    LeaveBreedingMarketHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCRMountFamilyTree => {
                                                    MountFamilyTreeHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCRRanchChat => {
                                                    RanchChatHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCRRanchCmdAction => {
                                                    RanchCmdActionHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCRRanchSnapshot => {
                                                    RanchSnapshotHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCRRequestNpcDressList => {
                                                    RequestNpcDressListHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCRRequestStorage => {
                                                    RequestStorageHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCRSearchStallion => {
                                                    SearchStallionHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCRTryBreeding => {
                                                    TryBreedingHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCRUpdateMountNickname => {
                                                    UpdateMountNicknameHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                CommandId::AcCmdCRWearEquipment => {
                                                    WearEquipmentHandler::handle_packet(
                                                        Arc::clone(&server),
                                                        Arc::clone(&session),
                                                        &packet,
                                                    )
                                                    .await
                                                }
                                                _ => Err("Unhandled command".into()),
                                            },
                                        };

                                        if let Err(e) = handle_result {
                                            // TODO: Implement handlers for these. Use them to detect half-closed connections
                                            let muted_packet = matches!(
                                                packet.command_id,
                                                CommandId::AcCmdCLHeartbeat
                                                    | CommandId::AcCmdCRHeartbeat
                                                    | CommandId::AcCmdCRRanchSnapshot
                                            );
                                            if !muted_packet {
                                                eprintln!(
                                                    "Failed to handle packet {:?}:\n\t{}\n\t{}\n",
                                                    packet.command_id,
                                                    e,
                                                    pretty_hex(&packet.payload)
                                                );
                                            }
                                        }

                                        Ok::<(), String>(()) // Continue processing packets in this session
                                    })
                                    .await
                                })
                                .await
                                .or_else(|join_err| {
                                    Err(format!("Couldn't join handler task: {}", join_err))
                                });

                            if let Err(handling_error) = handle_result {
                                eprintln!("/!\\ CONNECTION CLOSED\n{}", handling_error);
                                break;
                            }
                        }

                        println!("Connection closed");
                        server.lock().await.sessions.remove(&peer_addr);
                    })
                });
            }
        });

        server_instance.lock().await.worker_task = Some(worker_task);

        // Return server instance while it runs its client handling task
        Ok(server_instance)
    }

    pub async fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        self.stop = true;
        if let Some(worker_task) = self.worker_task.as_mut() {
            worker_task.await?;
        }
        Ok(())
    }
}

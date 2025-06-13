use std::{error::Error, io::Cursor, sync::Arc};

use deku::{DekuError, DekuWriter, writer::Writer};
use pretty_hex::pretty_hex;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
    sync::Mutex,
    task::JoinHandle,
};

use crate::{
    commands::Command,
    database::Database,
    entities::account::Account,
    handlers::{
        PacketHandler,
        lobby::{
            login::LoginHandler, request_league_info::RequestLeagueInfoHandler,
            show_inventory::ShowInventoryHandler,
        },
    },
    packet::{CommandId, MAX_BUFFER_SIZE, Packet, PacketScrambler},
};

pub struct Session {
    buf: [u8; MAX_BUFFER_SIZE],
    scrambler: PacketScrambler,
    socket: TcpStream,

    pub account: Option<Account>,
}
impl Session {
    fn new(socket: TcpStream) -> Self {
        Session {
            buf: [0u8; MAX_BUFFER_SIZE],
            scrambler: PacketScrambler { xor_key: 0 },
            socket,

            account: None,
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
        println!(
            ">>> Sent command {:?}:\n\tLength: {} ({:#x}) bytes\n{}\n\n",
            T::ID,
            packet.payload.len(),
            packet.payload.len(),
            formatted_command
        );
        Ok(())
    }

    pub async fn send_packet(&mut self, packet: &Packet) -> Result<(), String> {
        self.do_send_packet(packet).await?;
        println!(
            ">>> Sent packet {:?}:\n\t{}\n\n",
            packet.command_id,
            pretty_hex(&packet.payload)
        );
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

pub struct Server {
    pub name: String,
    pub database: Arc<Mutex<Database>>,
    tcp_server_task: Option<JoinHandle<()>>,
    stop: bool,
}
impl Server {
    pub async fn new(
        name: String,
        addr: &str,
        database: Arc<Mutex<Database>>,
    ) -> Result<Arc<Mutex<Server>>, Box<dyn Error>> {
        let tcp_listener = TcpListener::bind(addr).await?;
        println!("Server \"{name}\" listening on: {addr}");

        let server = Arc::new(Mutex::new(Server {
            name,
            database: Arc::clone(&database),
            tcp_server_task: None,
            stop: false,
        }));

        let server_clone = Arc::clone(&server);
        tokio::spawn(async move {
            while !server_clone.lock().await.stop {
                // Asynchronously wait for an inbound socket.
                let accept_result = tcp_listener.accept().await;
                if let Err(e) = accept_result {
                    eprintln!("Failed to accept socket: {}", e);
                    return;
                }
                let (socket, _) = accept_result.unwrap();

                // And this is where much of the magic of this server happens. We
                // crucially want all clients to make progress concurrently, rather than
                // blocking one on completion of another. To achieve this we use the
                // `tokio::spawn` function to execute the work in the background.
                //
                // Essentially here we're executing a new task to run concurrently,
                // which will allow all of our clients to be processed concurrently.
                let server_clone_clone = Arc::clone(&server_clone);
                tokio::spawn(async move {
                    println!("New connection established");
                    let mut session = Session::new(socket);
                    // In a loop, read data from the socket and write the data back.
                    while !server_clone_clone.lock().await.stop {
                        // Receive the next packet
                        let packet_result = session.recv_packet().await;
                        let packet = match packet_result {
                            Ok(pkt) => pkt,
                            Err(err) => {
                                eprintln!("Failed to receive packet: {}", err);
                                break;
                            }
                        };

                        println!(
                            "<<< Recv command {:?}:\n\t{}\n",
                            packet.command_id,
                            pretty_hex(&packet.payload)
                        );

                        let handle_result = match packet.command_id {
                            CommandId::AcCmdCLLogin => {
                                LoginHandler::handle_packet(
                                    Arc::clone(&server_clone_clone),
                                    &mut session,
                                    &packet,
                                )
                                .await
                            }
                            CommandId::AcCmdCLShowInventory => {
                                ShowInventoryHandler::handle_packet(
                                    Arc::clone(&server_clone_clone),
                                    &mut session,
                                    &packet,
                                )
                                .await
                            }
                            CommandId::AcCmdCLRequestLeagueInfo => {
                                RequestLeagueInfoHandler::handle_packet(
                                    Arc::clone(&server_clone_clone),
                                    &mut session,
                                    &packet,
                                )
                                .await
                            }
                            _ => Err(format!("Unhandled command: {:?}", packet.command_id)),
                        };

                        if let Err(e) = handle_result {
                            eprintln!("Failed to handle packet: {:?}", e);
                        }
                    }
                    println!("Connection closed");
                });
            }
        });
        Ok(server)
    }

    pub async fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        self.stop = true;
        if let Some(tcp_server_task) = self.tcp_server_task.as_mut() {
            tcp_server_task.await?;
        }
        Ok(())
    }
}

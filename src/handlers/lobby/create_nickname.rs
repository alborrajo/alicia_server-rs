use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::{lobby::{
        create_nickname::{CreateNickname, CreateNicknameCancel},
        show_inventory::ShowInventoryOk,
    }, shared::horse::{self, Horse, Mastery, Stats, Vals0, Vals1}},
    database::{character::insert_character, horse::insert_horse},
    entities::character::Character,
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct CreateNicknameHandler {}
impl CommandHandler for CreateNicknameHandler {
    type CommandType = CreateNickname;
    async fn handle_command(
        server: Arc<Mutex<Server>>,
        session: &mut Session,
        command: &Self::CommandType,
    ) -> Result<(), String> {
        let server = Arc::clone(&server);
        let database = Arc::clone(&server.lock().await.database);

        let account = session
            .account
            .as_ref()
            .ok_or("Attempted to create a character while not logged in")?;

        let result = database
            .lock()
            .await
            .run_in_transaction(async |transaction| {
                let character_id = account.member_no; // TODO: Autogenerate
                let mount_uid = account.member_no; // TODO: Autogenerate

                let character = 
                    Character {
                        character_id,
                        nickname: command
                            .nickname
                            .clone()
                            .into_string()
                            .map_err(|e| format!("Failed to convert nickname to String: {}", e))?,
                        mount_uid: mount_uid,
                        character: command.character.clone(),
                        create_character_unk0: command.unk0,
                    };
                insert_character(
                    transaction,
                    account.member_no,
                    &character
                )
                .await?;

                // Default horse
                let mount = Horse {
                    uid: mount_uid,
                    tid: 20001,
                    name: c"idontunderstand".to_owned(),
                    parts: horse::Parts {
                        skin_id: 1,
                        mane_id: 4,
                        tail_id: 4,
                        face_id: 5,
                    },
                    appearance: horse::Appearance {
                        scale: 0,
                        leg_length: 0,
                        leg_volume: 0,
                        body_length: 0,
                        body_volume: 0,
                    },
                    stats: Stats {
                        agility: 9,
                        control: 9,
                        speed: 9,
                        strength: 9,
                        spirit: 9,
                    },
                    rating: 0,
                    class: 21,
                    class_progress: 1,
                    grade: 5,
                    growth_points: 0,
                    vals0: Vals0 {
                        stamina: 65535,
                        attractiveness: 65535,
                        hunger: 65535,
                        val0: 0,
                        val1: 1000,
                        val2: 0,
                        val3: 0,
                        val4: 0,
                        val5: 1000,
                        val6: 30,
                        val7: 10,
                        val8: 10,
                        val9: 10,
                        val10: 0,
                    },
                    vals1: Vals1 {
                        val0: 0,
                        val1: 0,
                        date_of_birth: 3097585636,
                        val3: 2,
                        val4: 0,
                        class_progression: 255,
                        val5: 0,
                        potential_level: 0,
                        has_potential: 0,
                        potential_value: 255,
                        val9: 0,
                        luck: 4,
                        has_luck: 0,
                        val12: 0,
                        fatigue: 0,
                        val14: 0,
                        emblem: 1,
                    },
                    mastery: Mastery {
                        spur_magic_count: 510,
                        jump_count: 1057,
                        sliding_time: 1528,
                        gliding_distance: 53156,
                    },
                    val16: 3097585636,
                    val17: 0,
                };
                insert_horse(transaction, character_id, &mount).await?;

                Ok((mount, character))
            })
            .await;

        match result {
            Ok((mount, character)) => {
                session.character = Some(character);
                session.mount = Some(mount);
                session
                    .send_command(ShowInventoryOk {
                        // TODO
                        ..Default::default()
                    }).await?;
                Ok(())
            },
            Err(e) => {
                session.send_command(CreateNicknameCancel { 
                    error: 0 // TODO 
                }).await?;
                Err(e)
            }
        }.map_err(|e| format!("Failed to create character: {:?}", e))
    }
}
impl_packet_handler!(CreateNicknameHandler);

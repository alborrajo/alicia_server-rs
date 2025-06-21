use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::{lobby::{
        create_nickname::{CreateNickname, CreateNicknameCancel},
        show_inventory::ShowInventoryOk,
    }, shared::horse::{self, Horse, Mastery, Stats, Vals0, Vals1}, LengthPrefixedVec},
    database::{character::{insert_character, update_character}, horse::insert_horse},
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
        session: Arc<Mutex<Session>>,
        command: &Self::CommandType,
    ) -> Result<(), String> {
        let server = Arc::clone(&server);
        let database = Arc::clone(&server.lock().await.database);
        let mut session = session.lock().await;

        let account = session
            .account
            .as_ref()
            .ok_or("Attempted to create a character while not logged in")?;

        let result = database
            .lock()
            .await
            .run_in_transaction(async |transaction| {
                let mut character = 
                    Character {
                        character_id: 0, // Will be set by the database
                        nickname: command
                            .nickname
                            .clone()
                            .into_string()
                            .map_err(|e| format!("Failed to convert nickname to String: {}", e))?,
                        mount_uid: 0, // Will be set later when the horse is created
                        character: command.character.clone(),
                        create_character_unk0: command.unk0,
                    };
                insert_character(
                    transaction,
                    account.member_no,
                    &mut character
                )
                .await?;

                // Default horse
                let mut mount = Horse {
                    uid: 0, // Will be set by the database
                    tid: 20001,
                    name: c"idontunderstand".to_owned(),
                    parts: horse::Parts::random(),
                    appearance: horse::Appearance::default(),
                    stats: Stats::default(),
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
                insert_horse(transaction, character.character_id, &mut mount).await?;

                character.mount_uid = mount.uid;
                update_character(transaction, &character).await?;

                Ok((mount, character))
            })
            .await;

        match result {
            Ok((mount, character)) => {
                session.character = Some(character);
                session.horses = Some(vec![mount.clone()]);
                session
                    .send_command(ShowInventoryOk {
                        horses: LengthPrefixedVec {
                            vec: vec![
                                mount
                            ]
                        },
                        items: LengthPrefixedVec::default(),
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

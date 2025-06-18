use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::lobby::{
        create_nickname::{CreateNickname, CreateNicknameCancel},
        show_inventory::ShowInventoryOk,
    },
    database::character::insert_character,
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
                let character = 
                    Character {
                        character_id: account.member_no, // TODO: Autogenerate
                        nickname: command
                            .nickname
                            .clone()
                            .into_string()
                            .map_err(|e| format!("Failed to convert nickname to String: {}", e))?,
                        character: command.character.clone(),
                        create_character_unk0: command.unk0,
                    };
                insert_character(
                    transaction,
                    account.member_no,
                    &character
                )
                .await?;
                Ok(character)
            })
            .await;

        match result {
            Ok(character) => {
                session.character = Some(character);
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

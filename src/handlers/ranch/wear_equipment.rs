use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::ranch::wear_equipment::{WearEquipment, WearEquipmentOk},
    database::character::update_character,
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct WearEquipmentHandler {}
impl CommandHandler for WearEquipmentHandler {
    type CommandType = WearEquipment;
    async fn handle_command(
        server: Arc<Mutex<Server>>,
        session: Arc<Mutex<Session>>,
        command: &Self::CommandType,
    ) -> Result<(), String> {
        let server = server.lock().await;
        {
            let mut session = session.lock().await;
            let character = session
                .character
                .as_mut()
                .ok_or("Player has no character")?;
            character.mount_uid = command.item_uid;

            let mut database = server.database.lock().await;
            database
                .run_in_transaction(async |transaction| {
                    update_character(transaction, character).await
                })
                .await
                .map_err(|e| format!("Failed to update mount: {}", e))?;
        }

        let response = WearEquipmentOk {
            item_uid: command.item_uid,
            member: command.member,
        };

        let ranch_uid = session.lock().await.ranch_id.ok_or("Player has no ranch")?;
        for ranch_session in server
            .ranches
            .get(&ranch_uid)
            .ok_or(format!("Ranch {} doesn't exist", ranch_uid))?
            .character_sessions
            .as_slice()
        {
            ranch_session
                .lock()
                .await
                .send_command(response.clone())
                .await
                .map_err(|e| format!("Failed to send response: {:?}", e))?;
        }
        Ok(())
    }
}
impl_packet_handler!(WearEquipmentHandler);

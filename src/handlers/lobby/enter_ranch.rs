use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::lobby::enter_ranch::{EnterRanch, EnterRanchOk},
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct EnterRanchHandler {}
impl CommandHandler for EnterRanchHandler {
    type CommandType = EnterRanch;
    async fn handle_command(
        server: Arc<Mutex<Server>>,
        session: &mut Session,
        command: &Self::CommandType,
    ) -> Result<(), String> {
        let character_id = session
            .character
            .as_ref()
            .map(|c| c.character_id)
            .ok_or("Character not found")?;
        if command.character_id != character_id {
            return Err(format!(
                "Character ID mismatch: expected {}, got {}",
                character_id, command.character_id
            ));
        }

        // TODO: Fetch ranch from the server

        let ranch_address = server
            .lock()
            .await
            .settings
            .ranch_server
            .announce_address
            .clone();

        let response = EnterRanchOk {
            ranch_uid: 1234,
            code: 0x11223344, // This should probably be the packet scrambler code. TODO: Change
            address: ranch_address,
        };
        session
            .send_command(response)
            .await
            .map_err(|e| format!("Failed to send response: {:?}", e))
    }
}
impl_packet_handler!(EnterRanchHandler);

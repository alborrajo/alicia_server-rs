use std::sync::Arc;

use deku::DekuContainerRead;
use tokio::sync::Mutex;

use crate::{
    commands::lobby::request_daily_quest_list::{RequestDailyQuestList, RequestDailyQuestListOk},
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct RequestDailyQuestListHandler {}
impl CommandHandler for RequestDailyQuestListHandler {
    type CommandType = RequestDailyQuestList;
    async fn handle_command(
        _server: Arc<Mutex<Server>>,
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

        let pcap_data: [u8; 8] = [232, 226, 6, 0, 0, 0, 0, 0];
        let mut response = RequestDailyQuestListOk::from_bytes((&pcap_data, 0))
            .map_err(|e| format!("Failed to deserialize pcap: {:?}", e))
            .map(|result| result.1)?;
        response.character_id = session
            .character
            .as_ref()
            .map(|c| c.character_id)
            .ok_or("Character not found")?;

        session
            .send_command(response)
            .await
            .map_err(|e| format!("Failed to send response: {:?}", e))
    }
}
impl_packet_handler!(RequestDailyQuestListHandler);

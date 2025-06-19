use std::sync::Arc;

use deku::DekuContainerRead;
use tokio::sync::Mutex;

use crate::{
    commands::{
        lobby::request_quest_list::{RequestQuestList, RequestQuestListOk},
        shared::character,
    },
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct RequestQuestListHandler {}
impl CommandHandler for RequestQuestListHandler {
    type CommandType = RequestQuestList;
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

        let pcap_data: [u8; 201] = [
            232, 226, 6, 0, 15, 0, 22, 43, 0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 3, 23, 43, 0, 0, 0, 0, 3,
            6, 0, 0, 0, 0, 3, 24, 43, 0, 0, 0, 0, 3, 2, 0, 0, 0, 0, 3, 27, 43, 0, 0, 0, 0, 3, 11,
            0, 0, 0, 0, 3, 28, 43, 0, 0, 0, 0, 3, 31, 0, 0, 0, 0, 3, 31, 43, 0, 0, 0, 0, 1, 10, 0,
            0, 0, 0, 3, 234, 46, 0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 3, 235, 46, 0, 0, 0, 0, 3, 2, 0, 0,
            0, 0, 3, 236, 46, 0, 0, 0, 0, 1, 3, 0, 0, 0, 0, 3, 210, 50, 0, 0, 0, 0, 1, 20, 0, 0, 0,
            0, 3, 186, 54, 0, 0, 0, 0, 3, 2, 0, 0, 0, 0, 3, 187, 54, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0,
            3, 188, 54, 0, 0, 0, 0, 3, 4, 0, 0, 0, 0, 3, 189, 54, 0, 0, 0, 0, 3, 4, 0, 0, 0, 0, 3,
            193, 54, 0, 0, 0, 0, 3, 6, 0, 0, 0, 0, 3,
        ];
        let mut response = RequestQuestListOk::from_bytes((&pcap_data, 0))
            .map_err(|e| format!("Failed to deserialize pcap: {:?}", e))
            .map(|result| result.1)?;
        response.character_id = character_id;

        session
            .send_command(response)
            .await
            .map_err(|e| format!("Failed to send response: {:?}", e))
    }
}
impl_packet_handler!(RequestQuestListHandler);

use std::sync::Arc;

use deku::DekuContainerRead;
use tokio::sync::Mutex;

use crate::{
    commands::lobby::achievement_complete_list::{
        AchievementCompleteList, AchievementCompleteListOk,
    },
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct AchievementCompleteListHandler {}
impl CommandHandler for AchievementCompleteListHandler {
    type CommandType = AchievementCompleteList;
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

        let pcap_data: [u8; 425] = [
            232, 226, 6, 0, 28, 0, 40, 78, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 41, 78, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 42, 78, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 43, 78, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 0, 0, 44, 78, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 171, 39, 0, 0, 0, 0, 1, 1, 0, 0,
            0, 0, 0, 172, 39, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 173, 39, 0, 0, 0, 0, 1, 1, 0, 0, 0,
            0, 0, 174, 39, 0, 0, 0, 0, 1, 244, 1, 0, 0, 0, 0, 175, 39, 0, 0, 0, 0, 1, 1, 0, 0, 0,
            0, 0, 176, 39, 0, 0, 0, 0, 1, 5, 0, 0, 0, 0, 0, 177, 39, 0, 0, 0, 0, 1, 3, 0, 0, 0, 0,
            0, 178, 39, 0, 0, 0, 0, 1, 2, 0, 0, 0, 0, 0, 179, 39, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
            180, 39, 0, 0, 0, 0, 1, 244, 1, 0, 0, 0, 0, 181, 39, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
            182, 39, 0, 0, 0, 0, 255, 0, 0, 0, 0, 255, 0, 183, 39, 0, 0, 0, 0, 1, 3, 0, 0, 0, 0, 0,
            184, 39, 0, 0, 0, 0, 1, 3, 0, 0, 0, 0, 0, 185, 39, 0, 0, 0, 0, 1, 3, 0, 0, 0, 0, 0,
            186, 39, 0, 0, 0, 0, 1, 3, 0, 0, 0, 0, 0, 187, 39, 0, 0, 0, 0, 1, 3, 0, 0, 0, 0, 0,
            188, 39, 0, 0, 0, 0, 1, 3, 0, 0, 0, 0, 0, 189, 39, 0, 0, 0, 0, 255, 0, 0, 0, 0, 255, 0,
            190, 39, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 191, 39, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
            192, 39, 0, 0, 0, 0, 255, 0, 0, 0, 0, 255, 0, 193, 39, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
            3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut response = AchievementCompleteListOk::from_bytes((&pcap_data, 0))
            .map_err(|e| format!("Failed to deserialize pcap: {:?}", e))
            .map(|result| result.1)?;
        response.character_id = character_id;

        session
            .send_command(response)
            .await
            .map_err(|e| format!("Failed to send response: {:?}", e))
    }
}
impl_packet_handler!(AchievementCompleteListHandler);

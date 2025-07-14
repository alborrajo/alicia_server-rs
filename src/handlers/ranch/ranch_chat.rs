use std::{ffi::CString, str::FromStr, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    commands::ranch::ranch_chat::{RanchChat, RanchChatNotify},
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct RanchChatHandler {}
impl CommandHandler for RanchChatHandler {
    type CommandType = RanchChat;
    async fn handle_command(
        server: Arc<Mutex<Server>>,
        session: Arc<Mutex<Session>>,
        command: &Self::CommandType,
    ) -> Result<(), String> {
        let author = {
            let session = session.lock().await;
            let character = session
                .character
                .as_ref()
                .ok_or("Session has no character")?;
            CString::from_str(&character.nickname)
                .map_err(|_| format!("Failed to convert \"{}\" to CString", character.nickname))?
        };
        let ranch_sessions = {
            let ranch_id = session
                .lock()
                .await
                .ranch_id
                .ok_or("Player is in no ranch")?;
            let server = server.lock().await;
            let ranch = server
                .ranches
                .get(&ranch_id)
                .ok_or(format!("No ranch found with id {}", ranch_id))?;
            ranch.character_sessions.clone()
        };

        let response = RanchChatNotify {
            author,
            message: command.message.to_owned(),
            is_blue: command.unk0,
            unk1: command.unk1,
        };

        for ranch_session in ranch_sessions {
            let mut ranch_session = ranch_session.lock().await;
            ranch_session
                .send_command(response.clone())
                .await
                .map_err(|e| format!("Failed to send response: {:?}", e))?;
        }
        Ok(())
    }
}
impl_packet_handler!(RanchChatHandler);

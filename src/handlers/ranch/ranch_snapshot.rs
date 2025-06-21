use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::ranch::ranch_snapshot::{RanchSnapshot, RanchSnapshotNotify},
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct RanchSnapshotHandler {}
impl CommandHandler for RanchSnapshotHandler {
    type CommandType = RanchSnapshot;
    async fn handle_command(
        server: Arc<Mutex<Server>>,
        session: Arc<Mutex<Session>>,
        command: &Self::CommandType,
    ) -> Result<(), String> {
        let (character_id, ranch_id) = {
            let session = session.lock().await;
            (
                session
                    .character
                    .as_ref()
                    .ok_or("Player doesn't have a character")?
                    .character_id,
                session.ranch_id.ok_or("Player is not in any ranch")?,
            )
        };

        let server = server.lock().await;
        let ranch = server
            .ranches
            .get(&ranch_id)
            .ok_or(format!("Couldn't find ranch with id {}", ranch_id))?;
        let horses_count = ranch
            .owner
            .lock()
            .await
            .horses
            .as_ref()
            .ok_or("Ranch owner has no horses")?
            .len();
        let mut ranch_index = 0;
        for (idx, ranch_session) in ranch.character_sessions.iter().enumerate() {
            if ranch_session
                .lock()
                .await
                .character
                .as_ref()
                .is_some_and(|c| c.character_id == character_id)
            {
                ranch_index = horses_count + idx;
                break;
            }
        }

        let response = RanchSnapshotNotify {
            ranch_index: ranch_index as u16,
            snapshot: command.snapshot.clone(),
        };

        // Send to everyone in the ranch except the sender
        for ranch_session in ranch.character_sessions.iter() {
            let mut session = ranch_session.lock().await;
            if session
                .character
                .as_ref()
                .is_some_and(|c| c.character_id != character_id)
            {
                session
                    .send_command(response.clone())
                    .await
                    .map_err(|e| format!("Failed to send response: {:?}", e))?;
            }
        }
        Ok(())
    }
}
impl_packet_handler!(RanchSnapshotHandler);

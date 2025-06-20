use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::ranch::ranch_snapshot::RanchSnapshot,
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct RanchSnapshotHandler {}
impl CommandHandler for RanchSnapshotHandler {
    type CommandType = RanchSnapshot;
    async fn handle_command(
        _server: Arc<Mutex<Server>>,
        _session: &mut Session,
        _command: &Self::CommandType,
    ) -> Result<(), String> {
        return Ok(());

        // let response = RanchSnapshotNotify {
        //     ranch_index: 1, // TODO: Determine the session's ranch index
        //     snapshot: command.snapshot.clone(),
        // };
        // // TODO: Send to everyone in the ranch except the sender
        // session
        //     .send_command(response)
        //     .await
        //     .map_err(|e| format!("Failed to send response: {:?}", e))
    }
}
impl_packet_handler!(RanchSnapshotHandler);

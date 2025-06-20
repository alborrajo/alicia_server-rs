use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::ranch::ranch_cmd_action::{RanchCmdAction, RanchCmdActionNotify},
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct RanchCmdActionHandler {}
impl CommandHandler for RanchCmdActionHandler {
    type CommandType = RanchCmdAction;
    async fn handle_command(
        _server: Arc<Mutex<Server>>,
        session: Arc<Mutex<Session>>,
        _command: &Self::CommandType,
    ) -> Result<(), String> {
        // TODO: Send to all clients in the ranch
        let response = RanchCmdActionNotify::default();
        session
            .lock()
            .await
            .send_command(response)
            .await
            .map_err(|e| format!("Failed to send response: {:?}", e))
    }
}
impl_packet_handler!(RanchCmdActionHandler);

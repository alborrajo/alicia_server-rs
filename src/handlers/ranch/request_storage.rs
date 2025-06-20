use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::ranch::request_storage::{RequestStorage, RequestStorageOk},
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct RequestStorageHandler {}
impl CommandHandler for RequestStorageHandler {
    type CommandType = RequestStorage;
    async fn handle_command(
        _server: Arc<Mutex<Server>>,
        session: &mut Session,
        command: &Self::CommandType,
    ) -> Result<(), String> {
        let response = RequestStorageOk {
            val0: command.val0,
            val1: command.val1,
            ..Default::default()
        };
        session
            .send_command(response)
            .await
            .map_err(|e| format!("Failed to send response: {:?}", e))
    }
}
impl_packet_handler!(RequestStorageHandler);

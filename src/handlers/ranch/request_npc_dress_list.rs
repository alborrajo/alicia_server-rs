use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::ranch::request_npc_dress_list::{RequestNpcDressList, RequestNpcDressListOk},
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct RequestNpcDressListHandler {}
impl CommandHandler for RequestNpcDressListHandler {
    type CommandType = RequestNpcDressList;
    async fn handle_command(
        _server: Arc<Mutex<Server>>,
        session: Arc<Mutex<Session>>,
        _command: &Self::CommandType,
    ) -> Result<(), String> {
        let response = RequestNpcDressListOk::default();
        session
            .lock()
            .await
            .send_command(response)
            .await
            .map_err(|e| format!("Failed to send response: {:?}", e))
    }
}
impl_packet_handler!(RequestNpcDressListHandler);

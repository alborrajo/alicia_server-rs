use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::lobby::get_messenger_info::{GetMessengerInfo, GetMessengerInfoOk},
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct GetMessengerInfoHandler {}
impl CommandHandler for GetMessengerInfoHandler {
    type CommandType = GetMessengerInfo;
    async fn handle_command(
        server: Arc<Mutex<Server>>,
        session: Arc<Mutex<Session>>,
        _command: &Self::CommandType,
    ) -> Result<(), String> {
        let messenger_address = server
            .lock()
            .await
            .settings
            .messenger_server
            .announce_address
            .clone();

        let response = GetMessengerInfoOk {
            code: 0, // This is likely for the packet scrambler. TODO: Use
            address: messenger_address,
        };
        session
            .lock()
            .await
            .send_command(response)
            .await
            .map_err(|e| format!("Failed to send response: {:?}", e))
    }
}
impl_packet_handler!(GetMessengerInfoHandler);

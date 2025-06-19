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
        session: &mut Session,
        command: &Self::CommandType,
    ) -> Result<(), String> {
        let (messenger_ip, messenger_port) = {
            let server = server.lock().await;
            (
                server.settings.messenger_server.announce_ip,
                server.settings.messenger_server.announce_port,
            )
        };
        let response = GetMessengerInfoOk {
            code: 0, // This is likely for the packet scrambler. TODO: Use
            ip: messenger_ip,
            port: messenger_port,
        };
        session
            .send_command(response)
            .await
            .map_err(|e| format!("Failed to send response: {:?}", e))
    }
}
impl_packet_handler!(GetMessengerInfoHandler);

use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::lobby::request_special_event_list::{
        RequestSpecialEventList, RequestSpecialEventListOk,
    },
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct RequestSpecialEventListHandler {}
impl CommandHandler for RequestSpecialEventListHandler {
    type CommandType = RequestSpecialEventList;
    async fn handle_command(
        server: Arc<Mutex<Server>>,
        session: &mut Session,
        command: &Self::CommandType,
    ) -> Result<(), String> {
        let response = RequestSpecialEventListOk::default();
        session
            .send_command(response)
            .await
            .map_err(|e| format!("Failed to send response: {:?}", e))
    }
}
impl_packet_handler!(RequestSpecialEventListHandler);

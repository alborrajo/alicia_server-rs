use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::ranch::breeding_failure_card::{BreedingFailureCard, BreedingFailureCardCancel},
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct BreedingFailureCardHandler {}
impl CommandHandler for BreedingFailureCardHandler {
    type CommandType = BreedingFailureCard;
    async fn handle_command(
        _server: Arc<Mutex<Server>>,
        session: Arc<Mutex<Session>>,
        _command: &Self::CommandType,
    ) -> Result<(), String> {
        // TODO: The thing
        let response = BreedingFailureCardCancel {};
        session
            .lock()
            .await
            .send_command(response)
            .await
            .map_err(|e| format!("Failed to send response: {:?}", e))
    }
}
impl_packet_handler!(BreedingFailureCardHandler);

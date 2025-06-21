use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::ranch::breeding_wishlist::{BreedingWishlist, BreedingWishlistOk},
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct BreedingWishlistHandler {}
impl CommandHandler for BreedingWishlistHandler {
    type CommandType = BreedingWishlist;
    async fn handle_command(
        _server: Arc<Mutex<Server>>,
        session: Arc<Mutex<Session>>,
        _command: &Self::CommandType,
    ) -> Result<(), String> {
        // TODO: Implement
        let response = BreedingWishlistOk::default();
        session
            .lock()
            .await
            .send_command(response)
            .await
            .map_err(|e| format!("Failed to send response: {:?}", e))
    }
}
impl_packet_handler!(BreedingWishlistHandler);

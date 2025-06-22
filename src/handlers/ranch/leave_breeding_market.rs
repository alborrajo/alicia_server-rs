use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::ranch::leave_breeding_market::LeaveBreedingMarket,
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct LeaveBreedingMarketHandler {}
impl CommandHandler for LeaveBreedingMarketHandler {
    type CommandType = LeaveBreedingMarket;
    async fn handle_command(
        _server: Arc<Mutex<Server>>,
        _session: Arc<Mutex<Session>>,
        _command: &Self::CommandType,
    ) -> Result<(), String> {
        // TODO: Idk, something, with this info. Apparently there's no response
        Ok(())
    }
}
impl_packet_handler!(LeaveBreedingMarketHandler);

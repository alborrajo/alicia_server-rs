use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::{
        LengthPrefixedVec,
        ranch::enter_breeding_market::{
            AvailableHorse, EnterBreedingMarket, EnterBreedingMarketOk,
        },
    },
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct EnterBreedingMarketHandler {}
impl CommandHandler for EnterBreedingMarketHandler {
    type CommandType = EnterBreedingMarket;
    async fn handle_command(
        _server: Arc<Mutex<Server>>,
        session: Arc<Mutex<Session>>,
        _command: &Self::CommandType,
    ) -> Result<(), String> {
        // TODO: Filter only valid horses
        let response = EnterBreedingMarketOk {
            available_horses: LengthPrefixedVec {
                vec: session
                    .lock()
                    .await
                    .horses
                    .as_ref()
                    .ok_or("Character has no horses")?
                    .iter()
                    .map(|h| AvailableHorse {
                        uid: h.uid,
                        tid: h.tid,
                        ..Default::default()
                    })
                    .collect(),
            },
        };
        session
            .lock()
            .await
            .send_command(response)
            .await
            .map_err(|e| format!("Failed to send response: {:?}", e))
    }
}
impl_packet_handler!(EnterBreedingMarketHandler);

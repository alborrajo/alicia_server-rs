use std::sync::Arc;

use deku::DekuContainerRead;
use tokio::sync::Mutex;

use crate::{
    commands::lobby::request_league_info::{RequestLeagueInfo, RequestLeagueInfoOk},
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct RequestLeagueInfoHandler {}
impl CommandHandler for RequestLeagueInfoHandler {
    type CommandType = RequestLeagueInfo;
    async fn handle_command(
        server: Arc<Mutex<Server>>,
        session: &mut Session,
        command: &Self::CommandType,
    ) -> Result<(), String> {
        let pcap_data: [u8; 29] = [
            0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x01, 0x01, 0x01, 0x00, 0x00, 0x34, 0x01,
            0x00,
        ];
        let pcap = RequestLeagueInfoOk::from_bytes((&pcap_data, 0))
            .map_err(|e| format!("Failed to deserialize pcap: {:?}", e))
            .map(|result| result.1)?;
        session
            .send_command(pcap)
            .await
            .map_err(|e| format!("Failed to send response: {:?}", e))
    }
}
impl_packet_handler!(RequestLeagueInfoHandler);

use crate::{
    handlers::PacketHandler,
    packet::{CommandId, Packet},
};

pub struct RequestLeagueInfoHandler {}
impl PacketHandler for RequestLeagueInfoHandler {
    const COMMAND_ID: CommandId = CommandId::AcCmdCLRequestLeagueInfo;

    async fn handle_packet(
        session: &mut crate::Session,
        packet: &crate::packet::Packet,
    ) -> Result<(), String> {
        session
            .send_packet(&Packet {
                command_id: CommandId::AcCmdCLRequestLeagueInfoOK,
                payload: vec![
                    0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x01, 0x01, 0x01, 0x00, 0x00,
                    0x34, 0x01, 0x00,
                ],
            })
            .await
    }
}

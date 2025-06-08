use crate::{
    Session,
    commands::Command,
    packet::{CommandId, Packet},
};

pub mod lobby;

pub trait PacketHandler {
    const COMMAND_ID: CommandId;
    async fn handle_packet(session: &mut Session, packet: &Packet) -> Result<(), String>;
}

pub trait CommandHandler {
    type CommandType: Command;
    async fn handle_command(
        session: &mut Session,
        command: &Self::CommandType,
    ) -> Result<(), String>;
}

#[macro_export]
macro_rules! impl_packet_handler {
    ($t:ty) => {
        use crate::commands::Command; // Bring the Command trait into scope
        impl crate::handlers::PacketHandler for $t {
            const COMMAND_ID: CommandId = <Self as CommandHandler>::CommandType::ID;
            async fn handle_packet(
                session: &mut crate::Session,
                packet: &crate::packet::Packet,
            ) -> Result<(), String> {
                let mut cursor = std::io::Cursor::new(&packet.payload);
                let mut reader = deku::reader::Reader::new(&mut cursor);
                let command =
                    <Self as CommandHandler>::CommandType::from_reader_with_ctx(&mut reader, ())
                        .map_err(|e| format!("Failed to deserialize command: {:?}", e))?;
                Self::handle_command(session, &command).await
            }
        }
    };
}

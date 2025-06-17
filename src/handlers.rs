use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::Command,
    packet::{CommandId, Packet},
    server::{Server, Session},
};

pub mod lobby;

pub trait PacketHandler {
    const COMMAND_ID: CommandId;
    async fn handle_packet(
        server: Arc<Mutex<Server>>,
        session: &mut Session,
        packet: &Packet,
    ) -> Result<(), String>;
}

// TODO: Log packet bytes

pub trait CommandHandler {
    type CommandType: Command;
    async fn handle_command(
        server: Arc<Mutex<Server>>,
        session: &mut Session,
        command: &Self::CommandType,
    ) -> Result<(), String>;
}

#[macro_export]
macro_rules! impl_packet_handler {
    ($t:ty) => {
        use crate::commands::Command;
        impl crate::handlers::PacketHandler for $t {
            const COMMAND_ID: crate::packet::CommandId = <Self as CommandHandler>::CommandType::ID;
            async fn handle_packet(
                server: std::sync::Arc<tokio::sync::Mutex<crate::server::Server>>,
                session: &mut crate::server::Session,
                packet: &crate::packet::Packet,
            ) -> Result<(), String> {
                let mut cursor = std::io::Cursor::new(&packet.payload);
                let mut reader = deku::reader::Reader::new(&mut cursor);
                use deku::DekuReader;
                let command =
                    <Self as CommandHandler>::CommandType::from_reader_with_ctx(&mut reader, ())
                        .map_err(|e| format!("Failed to deserialize command: {:?}", e))?;
                println!(
                    "<<< Recv command {:?}:\n\tLength: {} ({:#x}) bytes\n{:#?}\n\n",
                    <Self as crate::handlers::PacketHandler>::COMMAND_ID,
                    packet.payload.len(),
                    packet.payload.len(),
                    command
                );
                Self::handle_command(server, session, &command).await
            }
        }
    };
}

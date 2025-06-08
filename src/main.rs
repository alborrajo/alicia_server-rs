mod commands;
mod entities;
mod handlers;
mod packet;

use packet::{CommandId, MAX_BUFFER_SIZE, Packet};

use deku::{DekuWriter, writer::Writer};
use pretty_hex::pretty_hex;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

use std::{error::Error, io::Cursor};

use crate::{
    handlers::{
        PacketHandler,
        lobby::{
            login::LoginHandler, request_league_info::RequestLeagueInfoHandler,
            show_inventory::ShowInventoryHandler,
        },
    },
    packet::PacketScrambler,
};

pub struct Session {
    buf: [u8; MAX_BUFFER_SIZE],
    scrambler: PacketScrambler,
    socket: TcpStream,
}
impl Session {
    fn new(socket: TcpStream) -> Self {
        Session {
            buf: [0u8; MAX_BUFFER_SIZE],
            scrambler: PacketScrambler { xor_key: 0 },
            socket,
        }
    }

    async fn recv_packet(&mut self) -> Result<Packet, String> {
        let mut packet = Packet::from_stream(&mut self.buf, &mut self.socket)
            .await
            .map_err(|e| format!("Error reading command: {}:\n\t{}", e, pretty_hex(&self.buf)))?;
        self.scrambler.scramble(&mut packet);
        Ok(packet)
    }

    async fn send_packet(&mut self, packet: &Packet) -> Result<(), String> {
        // Outgoing commands aren't scrambled, so we can write directly to the buffer
        let written_bytes = {
            let mut cursor = Cursor::new(&mut self.buf[..]);
            let mut writer = Writer::new(&mut cursor);
            packet
                .to_writer(&mut writer, ())
                .map_err(|err| format!("Error serializing command: {}", err))?;
            writer.bits_written / 8
        };
        let written_bytes = &self.buf[0..written_bytes];
        self.socket
            .write(&written_bytes)
            .await
            .map_err(|err| format!("Error sending command: {:?}", err))?;
        println!(
            ">>> Sent command {:?}:\n\t{}\n\n",
            packet.command_id,
            pretty_hex(&packet.payload)
        );
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.
    let addr = "0.0.0.0:10030";
    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {addr}");

    loop {
        // Asynchronously wait for an inbound socket.
        let (mut socket, _) = listener.accept().await?;

        // And this is where much of the magic of this server happens. We
        // crucially want all clients to make progress concurrently, rather than
        // blocking one on completion of another. To achieve this we use the
        // `tokio::spawn` function to execute the work in the background.
        //
        // Essentially here we're executing a new task to run concurrently,
        // which will allow all of our clients to be processed concurrently.
        tokio::spawn(async move {
            println!("New connection established");
            let mut session = Session::new(socket);
            // In a loop, read data from the socket and write the data back.
            loop {
                // Receive the next packet
                let packet_result = session.recv_packet().await;
                let mut packet = match packet_result {
                    Ok(pkt) => pkt,
                    Err(err) => {
                        eprintln!("Failed to receive packet: {}", err);
                        break;
                    }
                };

                println!(
                    "<<< Recv command {:?}:\n\t{}\n",
                    packet.command_id,
                    pretty_hex(&packet.payload)
                );

                let handle_result = match packet.command_id {
                    CommandId::AcCmdCLLogin => {
                        LoginHandler::handle_packet(&mut session, &packet).await
                    }
                    CommandId::AcCmdCLShowInventory => {
                        ShowInventoryHandler::handle_packet(&mut session, &packet).await
                    }
                    CommandId::AcCmdCLRequestLeagueInfo => {
                        RequestLeagueInfoHandler::handle_packet(&mut session, &packet).await
                    }
                    _ => Err(format!("Unhandled command: {:?}", packet.command_id)),
                };

                if let Err(e) = handle_result {
                    eprintln!("Failed to handle packet: {:?}", e);
                }
            }
            println!("Connection closed");
        });
    }
}

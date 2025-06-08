mod commands;
mod entities;
mod handlers;
mod packet;

use packet::{CommandId, MAX_BUFFER_SIZE, Packet};

use deku::{DekuWriter, writer::Writer};
use pretty_hex::pretty_hex;
use tokio::{io::AsyncWriteExt, net::TcpListener};

use std::{error::Error, io::Cursor};

use crate::{
    commands::{
        LengthPrefixedVec,
        lobby::login::{
            KeyboardOption, KeyboardOptions, LoginOk, MacroOptions, Options, Val5, Val5Val1, Val7,
            Val7Value, Val9, Val11, Val12, Val13,
        },
    },
    entities::{
        character::{
            self, AgeGroup, AnotherPlayerRelatedThing, Character, Gender, PlayerRelatedThing,
            YetAnotherPlayerRelatedThing,
        },
        horse::{self, Horse, Mastery, Stats, Vals0, Vals1},
        item::Item,
        win_file_time::WinFileTime,
    },
    packet::PacketScrambler,
};

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
            let mut scrambler = PacketScrambler { xor_key: 0 };
            // In a loop, read data from the socket and write the data back.
            loop {
                let mut buf = vec![0; MAX_BUFFER_SIZE as usize];
                let command = Packet::from_stream(&mut buf, &mut socket).await;
                if let Err(e) = command {
                    eprintln!("Error reading command: {}:\n\t{}", e, pretty_hex(&buf));
                    break;
                }

                // Process the command here
                let mut packet = command.unwrap();
                scrambler.scramble(&mut packet);
                println!(
                    "<<< Recv command {:?}:\n\t{}\n",
                    packet.command_id,
                    pretty_hex(&packet.payload)
                );

                let mut send_result = None;

                if packet.command_id == CommandId::AcCmdCLLogin {
                    let command = LoginOk {
                        lobby_time: WinFileTime {
                            low_date_time: 3599221550,
                            high_date_time: 31183665,
                        },
                        val0: 829332,
                        self_uid: 451304,
                        nick_name: c"rgnt".to_owned(),
                        motd: c"Welcome to Story of Alicia!".to_owned(),
                        profile_gender: Gender::Boy,
                        status: c"This person is mentally unstable".to_owned(),
                        character_equipment: LengthPrefixedVec {
                            vec: vec![Item {
                                uid: 1,
                                tid: 30008,
                                val: 0,
                                count: 1,
                            }],
                        },
                        mount_equipment: LengthPrefixedVec {
                            vec: vec![Item {
                                uid: 33574440,
                                tid: 20008,
                                val: 0,
                                count: 1,
                            }],
                        },
                        level: 161,
                        carrots: 255,
                        val1: 24880,
                        val2: 255,
                        val3: 255,
                        options: Options {
                            keyboard_options: Some(KeyboardOptions {
                                bindings: LengthPrefixedVec {
                                    vec: vec![
                                        KeyboardOption {
                                            index: 1,
                                            r#type: 22,
                                            key: 87,
                                        },
                                        KeyboardOption {
                                            index: 2,
                                            r#type: 21,
                                            key: 65,
                                        },
                                        KeyboardOption {
                                            index: 3,
                                            r#type: 23,
                                            key: 68,
                                        },
                                        KeyboardOption {
                                            index: 4,
                                            r#type: 24,
                                            key: 83,
                                        },
                                        KeyboardOption {
                                            index: 5,
                                            r#type: 18,
                                            key: 19,
                                        },
                                        KeyboardOption {
                                            index: 6,
                                            r#type: 130,
                                            key: 131,
                                        },
                                        KeyboardOption {
                                            index: 7,
                                            r#type: 32,
                                            key: 47,
                                        },
                                        KeyboardOption {
                                            index: 8,
                                            r#type: 70,
                                            key: 0,
                                        },
                                        KeyboardOption {
                                            index: 9,
                                            r#type: 82,
                                            key: 0,
                                        },
                                        KeyboardOption {
                                            index: 10,
                                            r#type: 25,
                                            key: 0,
                                        },
                                        KeyboardOption {
                                            index: 11,
                                            r#type: 15,
                                            key: 0,
                                        },
                                        KeyboardOption {
                                            index: 12,
                                            r#type: 67,
                                            key: 0,
                                        },
                                    ],
                                },
                            }),
                            macro_options: Some(MacroOptions {
                                macros: [
                                    c"/wink/wave".to_owned(),
                                    c"Thank you! /heart".to_owned(),
                                    c"/fire/fire/fire Fire! /fire/fire/fire".to_owned(),
                                    c"/sad/cry Sorry! /cry/sad".to_owned(),
                                    c"/-tada Congratulations!!! /tada".to_owned(),
                                    c"/clap Good Game! /-clap".to_owned(),
                                    c"Be right back! Please wait for me! /wink".to_owned(),
                                    c"See you! /smile/wave".to_owned(),
                                ],
                            }),
                            value_options: Some(100),
                        },
                        age_group: AgeGroup::Adult,
                        hide_age: 0,
                        val5: LengthPrefixedVec {
                            vec: vec![
                                Val5 {
                                    val0: 24,
                                    val1: LengthPrefixedVec {
                                        vec: vec![Val5Val1 { val0: 2, val1: 1 }],
                                    },
                                },
                                Val5 {
                                    val0: 31,
                                    val1: LengthPrefixedVec {
                                        vec: vec![Val5Val1 { val0: 2, val1: 1 }],
                                    },
                                },
                                Val5 {
                                    val0: 35,
                                    val1: LengthPrefixedVec {
                                        vec: vec![Val5Val1 { val0: 2, val1: 1 }],
                                    },
                                },
                                Val5 {
                                    val0: 41,
                                    val1: LengthPrefixedVec {
                                        vec: vec![Val5Val1 { val0: 2, val1: 1 }],
                                    },
                                },
                                Val5 {
                                    val0: 42,
                                    val1: LengthPrefixedVec {
                                        vec: vec![Val5Val1 { val0: 2, val1: 1 }],
                                    },
                                },
                                Val5 {
                                    val0: 43,
                                    val1: LengthPrefixedVec {
                                        vec: vec![Val5Val1 { val0: 2, val1: 1 }],
                                    },
                                },
                                Val5 {
                                    val0: 46,
                                    val1: LengthPrefixedVec {
                                        vec: vec![Val5Val1 { val0: 2, val1: 1 }],
                                    },
                                },
                            ],
                        },
                        val6: c"".to_owned(),
                        address: 16777343,
                        port: 10030,
                        scrambling_constant: 0,
                        character: Character {
                            parts: character::Parts {
                                char_id: 10,
                                mouth_serial_id: 1,
                                face_serial_id: 2,
                                val0: 1,
                            },
                            appearance: character::Appearance {
                                val0: 65535,
                                head_size: 4,
                                height: 8,
                                thigh_volume: 8,
                                leg_volume: 8,
                                val1: 255,
                            },
                        },
                        horse: Horse {
                            uid: 91857814,
                            tid: 20001,
                            name: c"idontunderstand".to_owned(),
                            parts: horse::Parts {
                                skin_id: 1,
                                mane_id: 4,
                                tail_id: 4,
                                face_id: 5,
                            },
                            appearance: horse::Appearance {
                                scale: 0,
                                leg_length: 0,
                                leg_volume: 0,
                                body_length: 0,
                                body_volume: 0,
                            },
                            stats: Stats {
                                agility: 9,
                                control: 9,
                                speed: 9,
                                strength: 9,
                                spirit: 9,
                            },
                            rating: 0,
                            class: 21,
                            class_progress: 1,
                            grade: 5,
                            growth_points: 0,
                            vals0: Vals0 {
                                stamina: 65535,
                                attractiveness: 65535,
                                hunger: 65535,
                                val0: 0,
                                val1: 1000,
                                val2: 0,
                                val3: 0,
                                val4: 0,
                                val5: 1000,
                                val6: 30,
                                val7: 10,
                                val8: 10,
                                val9: 10,
                                val10: 0,
                            },
                            vals1: Vals1 {
                                val0: 0,
                                val1: 0,
                                date_of_birth: 3097585636,
                                val3: 2,
                                val4: 0,
                                class_progression: 255,
                                val5: 0,
                                potential_level: 0,
                                has_potential: 0,
                                potential_value: 255,
                                val9: 0,
                                luck: 4,
                                has_luck: 0,
                                val12: 0,
                                fatigue: 0,
                                val14: 0,
                                emblem: 1,
                            },
                            mastery: Mastery {
                                spur_magic_count: 510,
                                jump_count: 1057,
                                sliding_time: 1528,
                                gliding_distance: 53156,
                            },
                            val16: 3097585636,
                            val17: 0,
                        },
                        val7: Val7 {
                            values: LengthPrefixedVec {
                                vec: vec![
                                    Val7Value { val0: 6, val1: 0 },
                                    Val7Value { val0: 15, val1: 4 },
                                    Val7Value { val0: 27, val1: 2 },
                                    Val7Value { val0: 30, val1: 0 },
                                    Val7Value { val0: 31, val1: 0 },
                                    Val7Value {
                                        val0: 37,
                                        val1: 30000,
                                    },
                                    Val7Value { val0: 53, val1: 4 },
                                    Val7Value { val0: 66, val1: 2 },
                                    Val7Value { val0: 67, val1: 4 },
                                    Val7Value { val0: 69, val1: 0 },
                                ],
                            },
                        },
                        bitfield: 3590,
                        val9: Val9 {
                            val0: 0,
                            val1: 0,
                            val2: 0,
                        },
                        val10: 0,
                        val11: Val11 {
                            val0: 4,
                            val1: 43,
                            val2: 4,
                        },
                        val12: Val12 {
                            values: LengthPrefixedVec { vec: vec![] },
                        },
                        val13: Val13 {
                            values: LengthPrefixedVec { vec: vec![] },
                        },
                        val14: 3390801883,
                        val15: PlayerRelatedThing {
                            val0: 0,
                            val1: 1,
                            val2: 0,
                            val3: c"".to_owned(),
                            val4: 0,
                            val5: 0,
                            val6: 0,
                        },
                        val16: 4,
                        val17: AnotherPlayerRelatedThing {
                            mount_uid: 91857814,
                            val1: 18,
                            val2: 24012772,
                        },
                        val18: 58,
                        val19: 910,
                        val20: 454,
                        val21: YetAnotherPlayerRelatedThing {
                            val0: 0,
                            val1: 0,
                            val2: c"".to_owned(),
                            val3: 0,
                        },
                    };
                    if let Ok(packet) = command.try_into() {
                        send_result = Some(send_packet(&mut buf, &mut socket, &packet).await);
                    } else {
                        eprintln!("Failed to serialize packet into a command");
                        break;
                    }
                }

                if packet.command_id == CommandId::AcCmdCLShowInventory {
                    send_result = Some(
                        send_packet(
                            &mut buf,
                            &mut socket,
                            &Packet {
                                command_id: CommandId::AcCmdCLShowInventoryOK,
                                payload: vec![
                                    0x1F, 0x4A, 0x75, 0x00, 0x02, 0x4A, 0x75, 0x00, 0x00, 0xB8,
                                    0x1B, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0xB0, 0x9A, 0x00,
                                    0x02, 0xB0, 0x9A, 0x00, 0x00, 0xB8, 0x1B, 0x01, 0x00, 0x01,
                                    0x00, 0x00, 0x00, 0x14, 0x9B, 0x00, 0x02, 0x14, 0x9B, 0x00,
                                    0x00, 0xB8, 0x1B, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x78,
                                    0x9B, 0x00, 0x02, 0x78, 0x9B, 0x00, 0x00, 0xB8, 0x1B, 0x01,
                                    0x00, 0x01, 0x00, 0x00, 0x00, 0x79, 0x9B, 0x00, 0x02, 0x79,
                                    0x9B, 0x00, 0x00, 0xB8, 0x1B, 0x01, 0x00, 0x01, 0x00, 0x00,
                                    0x00, 0x7A, 0x9B, 0x00, 0x02, 0x7A, 0x9B, 0x00, 0x00, 0xB8,
                                    0x1B, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x7B, 0x9B, 0x00,
                                    0x02, 0x7B, 0x9B, 0x00, 0x00, 0xB8, 0x1B, 0x01, 0x00, 0x01,
                                    0x00, 0x00, 0x00, 0x7C, 0x9B, 0x00, 0x02, 0x7C, 0x9B, 0x00,
                                    0x00, 0xB8, 0x1B, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x7D,
                                    0x9B, 0x00, 0x02, 0x7D, 0x9B, 0x00, 0x00, 0xB8, 0x1B, 0x01,
                                    0x00, 0x01, 0x00, 0x00, 0x00, 0x7E, 0x9B, 0x00, 0x02, 0x7E,
                                    0x9B, 0x00, 0x00, 0xB8, 0x1B, 0x01, 0x00, 0x01, 0x00, 0x00,
                                    0x00, 0x7F, 0x9B, 0x00, 0x02, 0x7F, 0x9B, 0x00, 0x00, 0xB8,
                                    0x1B, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x80, 0x9B, 0x00,
                                    0x02, 0x80, 0x9B, 0x00, 0x00, 0xB8, 0x1B, 0x01, 0x00, 0x01,
                                    0x00, 0x00, 0x00, 0x81, 0x9B, 0x00, 0x02, 0x81, 0x9B, 0x00,
                                    0x00, 0xB8, 0x1B, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0xE6,
                                    0x9B, 0x00, 0x02, 0xE6, 0x9B, 0x00, 0x00, 0xB8, 0x1B, 0x01,
                                    0x00, 0x01, 0x00, 0x00, 0x00, 0xE7, 0x9B, 0x00, 0x02, 0xE7,
                                    0x9B, 0x00, 0x00, 0xB8, 0x1B, 0x01, 0x00, 0x01, 0x00, 0x00,
                                    0x00, 0xE8, 0x9B, 0x00, 0x02, 0xE8, 0x9B, 0x00, 0x00, 0xB8,
                                    0x1B, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0xE9, 0x9B, 0x00,
                                    0x02, 0xE9, 0x9B, 0x00, 0x00, 0xB8, 0x1B, 0x01, 0x00, 0x01,
                                    0x00, 0x00, 0x00, 0x42, 0x9C, 0x00, 0x02, 0x42, 0x9C, 0x00,
                                    0x00, 0xB8, 0x1B, 0x01, 0x00, 0x06, 0x00, 0x00, 0x00, 0x29,
                                    0xA0, 0x00, 0x02, 0x29, 0xA0, 0x00, 0x00, 0xB8, 0x1B, 0x01,
                                    0x00, 0x1C, 0x00, 0x00, 0x00, 0x2A, 0xA0, 0x00, 0x02, 0x2A,
                                    0xA0, 0x00, 0x00, 0xB8, 0x1B, 0x01, 0x00, 0x0A, 0x00, 0x00,
                                    0x00, 0x2B, 0xA0, 0x00, 0x02, 0x2B, 0xA0, 0x00, 0x00, 0xB8,
                                    0x1B, 0x01, 0x00, 0x10, 0x00, 0x00, 0x00, 0x2C, 0xA0, 0x00,
                                    0x02, 0x2C, 0xA0, 0x00, 0x00, 0xB8, 0x1B, 0x01, 0x00, 0x0A,
                                    0x00, 0x00, 0x00, 0x2E, 0xA0, 0x00, 0x02, 0x2E, 0xA0, 0x00,
                                    0x00, 0xB8, 0x1B, 0x01, 0x00, 0x21, 0x00, 0x00, 0x00, 0x2F,
                                    0xA0, 0x00, 0x02, 0x2F, 0xA0, 0x00, 0x00, 0xB8, 0x1B, 0x01,
                                    0x00, 0x0A, 0x00, 0x00, 0x00, 0x30, 0xA0, 0x00, 0x02, 0x30,
                                    0xA0, 0x00, 0x00, 0xB8, 0x1B, 0x01, 0x00, 0x08, 0x00, 0x00,
                                    0x00, 0x31, 0xA0, 0x00, 0x02, 0x31, 0xA0, 0x00, 0x00, 0xB8,
                                    0x1B, 0x01, 0x00, 0x06, 0x00, 0x00, 0x00, 0x11, 0xA4, 0x00,
                                    0x02, 0x11, 0xA4, 0x00, 0x00, 0xB8, 0x1B, 0x01, 0x00, 0x18,
                                    0x00, 0x00, 0x00, 0xE1, 0xAB, 0x00, 0x02, 0xE1, 0xAB, 0x00,
                                    0x00, 0xB8, 0x1B, 0x01, 0x00, 0x05, 0x00, 0x00, 0x00, 0xE5,
                                    0xAB, 0x00, 0x02, 0xE5, 0xAB, 0x00, 0x00, 0xB8, 0x1B, 0x01,
                                    0x00, 0x03, 0x00, 0x00, 0x00, 0xC9, 0xAF, 0x00, 0x02, 0xC9,
                                    0xAF, 0x00, 0x00, 0xB8, 0x1B, 0x01, 0x00, 0x02, 0x00, 0x00,
                                    0x00, 0x94, 0x5F, 0x01, 0x02, 0x94, 0x5F, 0x01, 0x00, 0x00,
                                    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
                                ],
                            },
                        )
                        .await,
                    );
                }

                if packet.command_id == CommandId::AcCmdCLRequestLeagueInfo {
                    send_result = Some(
                        send_packet(
                            &mut buf,
                            &mut socket,
                            &Packet {
                                command_id: CommandId::AcCmdCLRequestLeagueInfoOK,
                                payload: vec![
                                    0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                    0x12, 0x01, 0x01, 0x01, 0x00, 0x00, 0x34, 0x01, 0x00,
                                ],
                            },
                        )
                        .await,
                    );
                }

                if let Some(Err(e)) = send_result {
                    eprintln!("{}", e);
                    break;
                }
            }
            println!("Connection closed");
        });
    }
}

async fn send_packet(
    buf: &mut [u8],
    socket: &mut tokio::net::TcpStream,
    command: &Packet,
) -> Result<(), String> {
    // Outgoing commands aren't scrambled, so we can write directly to the buffer
    let written_bytes = {
        let mut cursor = Cursor::new(&mut buf[..]);
        let mut writer = Writer::new(&mut cursor);
        command
            .to_writer(&mut writer, ())
            .map_err(|err| format!("Error serializing command: {}", err))?;
        writer.bits_written / 8
    };
    let written_bytes = &buf[0..written_bytes];
    socket
        .write(&written_bytes)
        .await
        .map_err(|err| format!("Error sending command: {:?}", err))?;
    println!(
        ">>> Sent command {:?}:\n\t{}\n\n",
        command.command_id,
        pretty_hex(&command.payload)
    );
    Ok(())
}

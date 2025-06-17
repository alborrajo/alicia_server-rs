use std::{error::Error, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    commands::{
        LengthPrefixedVec,
        lobby::login::{
            KeyboardOption, KeyboardOptions, Login, LoginCancel, LoginCancelReason, LoginOk,
            MacroOptions, Options, Val5, Val5Val1, Val7, Val7Value, Val9, Val11, Val12, Val13,
        },
        shared::{
            character::{
                self, AgeGroup, AnotherPlayerRelatedThing, Character, Gender, PlayerRelatedThing,
                YetAnotherPlayerRelatedThing,
            },
            horse::{self, Horse, Mastery, Stats, Vals0, Vals1},
            item::Item,
            win_file_time::WinFileTime,
        },
    },
    database::account::{add_account, get_account},
    entities::account::Account,
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct LoginHandler {}
impl CommandHandler for LoginHandler {
    type CommandType = Login;
    async fn handle_command(
        server: Arc<Mutex<Server>>,
        session: &mut Session,
        command: &Self::CommandType,
    ) -> Result<(), String> {
        let login_id = command
            .login_id
            .to_str()
            .map_err(|e| format!("Couldn't read login id: {}", e))?
            .to_owned();
        let auth_key = command
            .auth_key
            .to_str()
            .map_err(|e| format!("Couldn't read auth key: {}", e))?
            .to_owned();

        let server = Arc::clone(&server);
        let database = Arc::clone(&server.lock().await.database);
        let account: Result<Account, Box<dyn Error>> = database
            .lock()
            .await
            .run_in_transaction(async |transaction| {
                let candidate_account = get_account(transaction, command.member_no).await;
                if let Ok(candidate_account) = candidate_account {
                    if candidate_account.auth_key == auth_key
                        && candidate_account.login_id == login_id
                    {
                        Ok(candidate_account)
                    } else {
                        Err(format!("Auth key didn't match",).into())
                    }
                } else {
                    let new_account = Account {
                        member_no: command.member_no,
                        login_id: login_id.clone(),
                        auth_key: auth_key.clone(),
                    };
                    add_account(transaction, &new_account)
                        .await
                        .map_err(|err| {
                            format!("Failed to insert account in the database:\n\t{}", err)
                        })?;
                    Ok(new_account)
                }
            })
            .await;

        match account {
            Ok(account) => {
                println!("Logged in as {}", account.login_id.as_str());
                session.account = Some(account);
            }
            Err(error) => {
                println!(
                    "Failed attempt to log in as '{}' (ID {}) with auth key '{}': {}",
                    login_id, command.member_no, auth_key, error
                );
                session
                    .send_command(LoginCancel {
                        reason: LoginCancelReason::InvalidUser,
                    })
                    .await
                    .map_err(|e| format!("Failed to send response: {:?}", e))?;
                return Ok(());
            }
        }

        // TODO: Fetch character

        let response = LoginOk {
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
        session
            .send_command(response)
            .await
            .map_err(|e| format!("Failed to send response: {:?}", e))
    }
}
impl_packet_handler!(LoginHandler);

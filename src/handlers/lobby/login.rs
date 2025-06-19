use std::{error::Error, ffi::CString, str::FromStr, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    commands::{
        LengthPrefixedVec,
        lobby::{
            create_nickname::CreateNicknameNotify,
            login::{
                KeyboardOption, KeyboardOptions, Login, LoginCancel, LoginCancelReason, LoginOk,
                MacroOptions, Options, Val5, Val5Val1, Val7, Val7Value, Val9, Val11, Val12, Val13,
            },
        },
        shared::{
            character::{
                AgeGroup, AnotherPlayerRelatedThing, Gender, PlayerRelatedThing,
                YetAnotherPlayerRelatedThing,
            },
            item::Item,
            win_file_time::WinFileTime,
        },
    },
    database::{
        account::{add_account, get_account},
        character::get_character_by_member_no,
        horse::get_horse_by_uid,
    },
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
                    let mut new_account = Account {
                        member_no: command.member_no,
                        login_id: login_id.clone(),
                        auth_key: auth_key.clone(),
                    };
                    add_account(transaction, &mut new_account)
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

        let character = database
            .lock()
            .await
            .run_in_transaction(async |transaction| {
                get_character_by_member_no(transaction, command.member_no).await
            })
            .await
            .map_err(|e| format!("Failed to fetch character: {}", e))?;

        let mount = if let Some(character) = &character {
            Some(
                database
                    .lock()
                    .await
                    .run_in_transaction(async |transaction| {
                        get_horse_by_uid(transaction, character.mount_uid).await
                    })
                    .await
                    .map_err(|e| format!("Failed to fetch horse: {}", e))?
                    .ok_or(format!(
                        "Failed to find horse with UID {} for character '{}' ({})",
                        character.mount_uid, character.nickname, character.character_id
                    ))?,
            )
        } else {
            None
        };

        // Generate packet scrambler key
        session.scrambler.xor_key = rand::random();

        let lobby_address = server
            .lock()
            .await
            .settings
            .lobby_server
            .announce_address
            .clone();

        session
            .send_command(LoginOk {
                lobby_time: WinFileTime {
                    low_date_time: 3599221550,
                    high_date_time: 31183665,
                },
                val0: 829332,
                self_uid: character
                    .as_ref()
                    .map(|c| c.character_id)
                    .unwrap_or_default(),
                nick_name: character
                    .as_ref()
                    .map_or(Ok(c"".to_owned()), |c| {
                        CString::from_str(c.nickname.as_str())
                    })
                    .map_err(|e| format!("Failed to convert nickname to CString: {}", e))?,
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
                lobby_server_address: lobby_address,
                scrambling_constant: session.scrambler.xor_key,
                character: character
                    .as_ref()
                    .map(|c| c.character.clone())
                    .unwrap_or_default(),
                horse: mount.as_ref().map(|h| h.clone()).unwrap_or_default(),
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
                val17: mount
                    .as_ref()
                    .map(|h| AnotherPlayerRelatedThing {
                        mount_uid: h.uid,
                        val1: 18,
                        val2: 24012772,
                    })
                    .unwrap_or_default(),
                val18: 58,
                val19: 910,
                val20: 454,
                val21: YetAnotherPlayerRelatedThing {
                    val0: 0,
                    val1: 0,
                    val2: c"".to_owned(),
                    val3: 0,
                },
            })
            .await
            .map_err(|e| format!("Failed to send login OK response: {}", e))?;

        if character.is_none() {
            session
                .send_command(CreateNicknameNotify {})
                .await
                .map_err(|e| format!("Failed to send create nickname notify: {}", e))?;
        }

        session.character = character;
        session.mount = mount;

        Ok(())
    }
}
impl_packet_handler!(LoginHandler);

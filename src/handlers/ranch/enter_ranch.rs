use std::{collections::hash_map::Entry, ffi::CString, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    commands::{
        LengthPrefixedVec,
        ranch::{
            RanchCharacter, RanchHorse, RanchUnk11,
            enter_ranch::{EnterRanch, EnterRanchNotify, EnterRanchOk},
        },
        shared::character::{
            AnotherPlayerRelatedThing, PlayerRelatedThing, YetAnotherPlayerRelatedThing,
        },
    },
    database::{character::get_character_by_id, horse::get_horses_by_character_id},
    handlers::CommandHandler,
    impl_packet_handler,
    ranch::Ranch,
    server::{Server, Session},
};

pub struct EnterRanchHandler {}
impl CommandHandler for EnterRanchHandler {
    type CommandType = EnterRanch;
    async fn handle_command(
        server: Arc<Mutex<Server>>,
        session: Arc<Mutex<Session>>,
        command: &Self::CommandType,
    ) -> Result<(), String> {
        // TODO: Validate command.otp

        let server = Arc::clone(&server);

        // Load player data from DB if just logging in
        {
            let database = Arc::clone(&server.lock().await.database);
            let mut session = session.lock().await;
            if session.character.is_none() || session.horses.is_none() {
                let (character, horses) = database
                    .lock()
                    .await
                    .run_in_transaction(async |transaction| {
                        let character = get_character_by_id(transaction, command.character_uid)
                            .await
                            .map_err(|e| {
                                format!(
                                    "Failed to fetch character with id {}: {}",
                                    command.character_uid, e
                                )
                            })?
                            .ok_or("Character not found".to_owned())?;
                        let horses = get_horses_by_character_id(transaction, command.character_uid)
                            .await
                            .map_err(|e| {
                                format!(
                                    "Failed to fetch horses for character {}: {}",
                                    command.character_uid, e
                                )
                            })?;
                        Ok((character, horses))
                    })
                    .await
                    .map_err(|e| format!("Failed to load character and horses: {}", e))?;
                session.character = Some(character.clone());
                session.horses = Some(horses.clone());
            }
        }

        let mut server = server.lock().await;
        let ranch = match server.ranches.entry(command.ranch_uid) {
            Entry::Occupied(ranch) => ranch.into_mut(),
            Entry::Vacant(entry) => entry.insert({
                let (nickname, character_id) = session
                    .lock()
                    .await
                    .character
                    .as_ref()
                    .map(|c| (c.nickname.to_owned(), c.character_id))
                    .ok_or("Session has no character loaded")?;
                Ranch {
                    name: format!("{}'s Ranch", nickname),
                    owner: Arc::clone(&session),
                    character_sessions: vec![],
                }
            }),
        };

        // Add new player to ranch
        ranch.character_sessions.push(Arc::clone(&session));

        let mut ranch_index = 0;

        let ranch_horses = {
            let ranch_owner = ranch.owner.lock().await;
            let ranch_owner_horses = ranch_owner
                .horses
                .as_ref()
                .ok_or("Ranch owner has no horses".to_owned())?;
            let ranch_owner_mount = ranch_owner
                .get_mount()
                .ok_or("Ranch owner has no mount".to_owned())?;
            let mut ranch_horses = Vec::new();
            for horse in ranch_owner_horses {
                if horse.uid != ranch_owner_mount.uid {
                    ranch_index = ranch_index + 1;
                    ranch_horses.push(RanchHorse {
                        ranch_index,
                        horse: horse.clone(),
                    });
                }
            }
            ranch_horses
        };

        let (ranch_characters, new_ranch_character) = {
            let mut new_ranch_character: Option<RanchCharacter> = None;
            let mut ranch_characters = Vec::new();
            for ranch_session in ranch.character_sessions.as_slice() {
                ranch_index = ranch_index + 1;
                let ranch_session = ranch_session.lock().await;
                let ranch_session_character = ranch_session
                    .character
                    .as_ref()
                    .ok_or("Ranch session has no character")?;
                let ranch_session_mount = ranch_session.get_mount().ok_or(format!(
                    "Ranch session with character id {} has no mount",
                    ranch_session_character.character_id
                ))?;
                let ranch_character = RanchCharacter {
                    uid: ranch_session_character.character_id,
                    name: CString::new(ranch_session_character.nickname.clone())
                        .map_err(|e| format!("Failed to convert nickname to CString: {}", e))?,
                    gender: ranch_session_character.character.parts.gender(),
                    unk0: 1,
                    unk1: 1,
                    description: c"Description".into(),
                    character: ranch_session_character.character.clone(),
                    mount: ranch_session_mount.clone(),
                    character_equipment: LengthPrefixedVec::default(),
                    player_related_thing: PlayerRelatedThing::default(),
                    ranch_index,
                    unk2: 0,
                    unk3: 0,
                    another_player_related_thing: AnotherPlayerRelatedThing {
                        mount_uid: ranch_session_mount.uid,
                        ..Default::default()
                    },
                    yet_another_player_related_thing: YetAnotherPlayerRelatedThing::default(),
                    unk4: 0,
                    unk5: 0,
                };
                if ranch_character.uid == command.character_uid {
                    new_ranch_character = Some(ranch_character.clone());
                }
                ranch_characters.push(ranch_character);
            }
            (ranch_characters, new_ranch_character.ok_or("What")?)
        };

        let response = EnterRanchOk {
            ranch_id: command.ranch_uid,
            unk0: c"Unk0".into(),
            ranch_name: CString::new(ranch.name.clone())
                .map_err(|e| "Failed to convert ranch name to CString")?,
            horses: LengthPrefixedVec { vec: ranch_horses },
            character: LengthPrefixedVec {
                vec: ranch_characters,
            },
            unk1: 0,
            unk2: 0,
            unk3: 0,
            unk4: LengthPrefixedVec { vec: vec![] },
            unk5: 0,
            unk6: 0,
            unk7: 0,
            unk8: 0,
            unk9: 0,
            unk10: Default::default(),
            unk11: RanchUnk11::default(),
            unk12: 0,
        };

        {
            let mut session = session.lock().await;
            session.scrambler.xor_key = 0; // TODO: Find out where in the response this gets set
            session.ranch_id = Some(command.ranch_uid);
            session
                .send_command(response)
                .await
                .map_err(|e| format!("Failed to send response: {:?}", e))?;
        }

        // Notify the old players of the new player entering
        let notify = EnterRanchNotify {
            character: new_ranch_character,
        };
        for ranch_session in ranch.character_sessions.as_slice() {
            let mut ranch_session = ranch_session.lock().await;
            if ranch_session
                .character
                .as_ref()
                .is_some_and(|c| c.character_id != command.character_uid)
            {
                ranch_session
                    .send_command(notify.clone())
                    .await
                    .map_err(|e| format!("Failed to send notify: {:?}", e).to_owned())?;
            }
        }

        Ok(())
    }
}
impl_packet_handler!(EnterRanchHandler);

use std::{ffi::CString, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    commands::{
        LengthPrefixedVec,
        ranch::{
            RanchCharacter, RanchHorse, RanchUnk11,
            enter_ranch::{EnterRanch, EnterRanchOk},
        },
        shared::character::{
            AnotherPlayerRelatedThing, PlayerRelatedThing, YetAnotherPlayerRelatedThing,
        },
    },
    database::{character::get_character_by_id, horse::get_horses_by_character_id},
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct EnterRanchHandler {}
impl CommandHandler for EnterRanchHandler {
    type CommandType = EnterRanch;
    async fn handle_command(
        server: Arc<Mutex<Server>>,
        session: &mut Session,
        command: &Self::CommandType,
    ) -> Result<(), String> {
        // TODO: Validate command.otp

        let server = Arc::clone(&server);
        let database = Arc::clone(&server.lock().await.database);
        let (character, horses, mount) = database
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
                let mount = horses
                    .iter()
                    .filter(|h| h.uid == character.mount_uid)
                    .next()
                    .cloned()
                    .ok_or(format!("Character {} has no mount", command.character_uid))?;
                Ok((character, horses, mount))
            })
            .await
            .map_err(|e| format!("Failed to load character and horses: {}", e))?;

        session.character = Some(character.clone());
        session.horses = Some(horses.clone());

        // TODO: Figure out where in the response the key is supposed to be sent
        session.scrambler.xor_key = 0;

        let mut ranch_index = 0;
        let response = EnterRanchOk {
            ranch_id: command.ranch_uid,
            unk0: c"Unk0".into(),
            ranch_name: c"TODO Ranch".into(),
            horses: LengthPrefixedVec {
                vec: horses
                    .iter()
                    .filter(|h| h.uid != mount.uid)
                    .map(|h| {
                        ranch_index = ranch_index + 1;
                        RanchHorse {
                            ranch_index,
                            horse: h.clone(),
                        }
                    })
                    .collect(),
            },
            character: LengthPrefixedVec {
                // TODO: Get ranch character's from the server state
                vec: vec![RanchCharacter {
                    uid: character.character_id,
                    name: CString::new(character.nickname.clone())
                        .map_err(|e| format!("Failed to convert nickname to CString: {}", e))?,
                    gender: character.character.parts.gender(),
                    unk0: 1,
                    unk1: 1,
                    description: c"Description".into(),
                    character: character.character.clone(),
                    mount: mount.clone(),
                    character_equipment: LengthPrefixedVec::default(),
                    player_related_thing: PlayerRelatedThing::default(),
                    ranch_index: ranch_index + 1,
                    unk2: 0,
                    unk3: 0,
                    another_player_related_thing: AnotherPlayerRelatedThing {
                        mount_uid: mount.uid,
                        ..Default::default()
                    },
                    yet_another_player_related_thing: YetAnotherPlayerRelatedThing::default(),
                    unk4: 0,
                    unk5: 0,
                }],
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
        session
            .send_command(response)
            .await
            .map_err(|e| format!("Failed to send response: {:?}", e))
    }
}
impl_packet_handler!(EnterRanchHandler);

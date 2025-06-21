use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::{
        ranch::try_breeding::{TryBreeding, TryBreedingOk},
        shared::horse::{Appearance, Horse, Mastery, Parts, Stats, Vals0, Vals1},
    },
    database::horse::insert_horse,
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct TryBreedingHandler {}
impl CommandHandler for TryBreedingHandler {
    type CommandType = TryBreeding;
    async fn handle_command(
        server: Arc<Mutex<Server>>,
        session: Arc<Mutex<Session>>,
        _command: &Self::CommandType,
    ) -> Result<(), String> {
        // TODO: Substract carrots from self, add carrots to the other player, update breeding count, remove listing, god knows how many other things...

        let character_id = session
            .lock()
            .await
            .character
            .as_ref()
            .ok_or("Player has no character")?
            .character_id;
        let mut new_horse = Horse {
            uid: 0, // Will be set by the database
            tid: 20001,
            name: c"".to_owned(), // Will be set in the update mount nickname handler
            parts: Parts::random(),
            appearance: Appearance::default(),
            stats: Stats::default(),
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
        };

        {
            let server = server.lock().await;
            let mut database = server.database.lock().await;
            database
                .run_in_transaction(async |transaction| {
                    insert_horse(transaction, character_id, &mut new_horse).await
                })
                .await
                .map_err(|e| format!("Failed to insert horse: {}", e).to_owned())?;
        }

        let response = TryBreedingOk {
            uid: new_horse.uid,
            tid: new_horse.tid,
            // wild guesses
            val: 0,
            count: 0,
            unk0: 0,
            parts: new_horse.parts.clone(),
            appearance: new_horse.appearance.clone(),
            stats: new_horse.stats.clone(),
            // wild guesses
            unk1: new_horse.vals1.val5,
            unk2: new_horse.vals1.potential_level,
            unk3: new_horse.vals1.has_potential,
            unk4: new_horse.vals1.potential_value,
            unk5: new_horse.vals1.val9,
            unk6: new_horse.vals1.luck,
            unk7: new_horse.vals1.has_luck,
            unk8: new_horse.vals1.val12,
            unk9: new_horse.vals1.fatigue,
            unk10: 0,
        };

        let mut session = session.lock().await;
        session
            .horses
            .as_mut()
            .ok_or("Character has no horses")?
            .push(new_horse);
        session
            .send_command(response)
            .await
            .map_err(|e| format!("Failed to send response: {:?}", e))
    }
}
impl_packet_handler!(TryBreedingHandler);

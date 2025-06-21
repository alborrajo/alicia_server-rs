use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::{
        LengthPrefixedVec,
        ranch::search_stallion::{SearchStallion, SearchStallionOk, Stallion},
        shared::horse::{Appearance, Parts, Stats},
    },
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct SearchStallionHandler {}
impl CommandHandler for SearchStallionHandler {
    type CommandType = SearchStallion;
    async fn handle_command(
        _server: Arc<Mutex<Server>>,
        session: Arc<Mutex<Session>>,
        _command: &Self::CommandType,
    ) -> Result<(), String> {
        // TODO: Fetch from DB
        let response = SearchStallionOk {
            unk0: 0,
            unk1: 0,
            stallions: LengthPrefixedVec {
                vec: vec![Stallion {
                    unk0: c"Unk0".to_owned(),
                    uid: 0x3004e21,
                    tid: 20001,
                    name: c"Name".to_owned(),
                    grade: 4,
                    chance: 0,
                    price: 1,
                    unk7: 0xFFFFFFFF,
                    unk8: 0xFFFFFFFF,
                    stats: Stats::default(),
                    parts: Parts::random(),
                    appearance: Appearance::default(),
                    unk11: 5,
                    coat_bonus: 0,
                }],
            },
        };
        session
            .lock()
            .await
            .send_command(response)
            .await
            .map_err(|e| format!("Failed to send response: {:?}", e))
    }
}
impl_packet_handler!(SearchStallionHandler);

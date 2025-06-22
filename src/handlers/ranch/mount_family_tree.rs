use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::{
        LengthPrefixedVec,
        ranch::mount_family_tree::{MountFamilyTree, MountFamilyTreeItem, MountFamilyTreeOk},
    },
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct MountFamilyTreeHandler {}
impl CommandHandler for MountFamilyTreeHandler {
    type CommandType = MountFamilyTree;
    async fn handle_command(
        _server: Arc<Mutex<Server>>,
        session: Arc<Mutex<Session>>,
        command: &Self::CommandType,
    ) -> Result<(), String> {
        // TODO: Fetch
        let response = MountFamilyTreeOk {
            uid: command.uid,
            items: LengthPrefixedVec {
                vec: vec![
                    MountFamilyTreeItem {
                        unk0: 0,
                        unk1: c"Parent0".to_owned(),
                        unk2: 2,
                        unk3: 3,
                    },
                    MountFamilyTreeItem {
                        unk0: 1,
                        unk1: c"Parent1".to_owned(),
                        unk2: 12,
                        unk3: 13,
                    },
                ],
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
impl_packet_handler!(MountFamilyTreeHandler);

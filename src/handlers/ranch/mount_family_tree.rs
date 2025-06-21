use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::ranch::mount_family_tree::{MountFamilyTree, MountFamilyTreeOk},
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
            ..Default::default()
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

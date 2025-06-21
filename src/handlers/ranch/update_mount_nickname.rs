use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    commands::ranch::update_mount_nickname::{UpdateMountNickname, UpdateMountNicknameOk},
    database::horse::update_horse,
    handlers::CommandHandler,
    impl_packet_handler,
    server::{Server, Session},
};

pub struct UpdateMountNicknameHandler {}
impl CommandHandler for UpdateMountNicknameHandler {
    type CommandType = UpdateMountNickname;
    async fn handle_command(
        server: Arc<Mutex<Server>>,
        session: Arc<Mutex<Session>>,
        command: &Self::CommandType,
    ) -> Result<(), String> {
        let mut session = session.lock().await;
        let horses = session.horses.as_mut().ok_or("Character has no horses")?;
        let horse = horses
            .iter_mut()
            .find(|h| h.uid == command.uid)
            .ok_or(format!("Couldn't find horse with uid {}", command.uid).to_owned())?;
        horse.name = command.nickname.to_owned();

        let server = server.lock().await;
        let mut database = server.database.lock().await;
        database
            .run_in_transaction(async |transaction| update_horse(transaction, horse).await)
            .await
            .map_err(|e| format!("Couldn't update horse: {}", e).to_owned())?;

        let response = UpdateMountNicknameOk {
            uid: horse.uid,
            nickname: horse.name.to_owned(),
            ..Default::default() // TODO
        };
        session
            .send_command(response)
            .await
            .map_err(|e| format!("Failed to send response: {:?}", e))
    }
}
impl_packet_handler!(UpdateMountNicknameHandler);

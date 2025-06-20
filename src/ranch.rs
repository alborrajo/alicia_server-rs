use std::sync::Arc;

use tokio::sync::Mutex;

use crate::server::Session;

pub struct Ranch {
    pub name: String,
    pub owner: Arc<Mutex<Session>>,
    pub character_sessions: Vec<(u32, Arc<Mutex<Session>>)>,
}

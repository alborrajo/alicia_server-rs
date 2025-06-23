use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};

use crate::commands::shared::address::Address;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub lobby_server: ServerSettings,
    pub ranch_server: ServerSettings,
    pub race_server: ServerSettings,
    pub messenger_server: ServerSettings,
    pub database: DatabaseSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    pub enabled: bool,
    pub bind_address: String,
    pub announce_address: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub url: Option<String>,
    pub wipe_on_startup: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            lobby_server: ServerSettings {
                enabled: true,
                bind_address: "0.0.0.0:10030".to_owned(),
                announce_address: Address {
                    ip: Ipv4Addr::new(192, 168, 1, 32),
                    port: 10030,
                },
            },
            ranch_server: ServerSettings {
                enabled: true,
                bind_address: "0.0.0.0:10031".to_owned(),
                announce_address: Address {
                    ip: Ipv4Addr::new(192, 168, 1, 32),
                    port: 10031,
                },
            },
            race_server: ServerSettings {
                enabled: true,
                bind_address: "0.0.0.0:10032".to_owned(),
                announce_address: Address {
                    ip: Ipv4Addr::new(192, 168, 1, 32),
                    port: 10032,
                },
            },
            messenger_server: ServerSettings {
                enabled: true,
                bind_address: "0.0.0.0:10033".to_owned(),
                announce_address: Address {
                    ip: Ipv4Addr::new(192, 168, 1, 32),
                    port: 10033,
                },
            },
            database: DatabaseSettings {
                url: None,
                wipe_on_startup: false,
            },
        }
    }
}

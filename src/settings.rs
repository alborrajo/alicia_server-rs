use std::net::Ipv4Addr;

#[derive(Debug, Clone)]
pub struct Settings {
    pub lobby_server: ServerSettings,
    pub ranch_server: ServerSettings,
    pub race_server: ServerSettings,
    pub messenger_server: ServerSettings,
    pub database: DatabaseSettings,
}

#[derive(Debug, Clone)]
pub struct ServerSettings {
    pub enabled: bool,
    pub bind_address: String,
    pub announce_ip: u32,
    pub announce_port: u16,
}

#[derive(Debug, Clone)]
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
                announce_ip: Ipv4Addr::new(127, 0, 0, 1).into(),
                announce_port: 10030,
            },
            ranch_server: ServerSettings {
                enabled: true,
                bind_address: "0.0.0.0:10031".to_owned(),
                announce_ip: Ipv4Addr::new(127, 0, 0, 1).into(),
                announce_port: 10031,
            },
            race_server: ServerSettings {
                enabled: true,
                bind_address: "0.0.0.0:10032".to_owned(),
                announce_ip: Ipv4Addr::new(127, 0, 0, 1).into(),
                announce_port: 10032,
            },
            messenger_server: ServerSettings {
                enabled: true,
                bind_address: "0.0.0.0:10033".to_owned(),
                announce_ip: Ipv4Addr::new(127, 0, 0, 1).into(),
                announce_port: 10033,
            },
            database: DatabaseSettings {
                url: None,
                wipe_on_startup: true,
            },
        }
    }
}

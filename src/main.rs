mod commands;
mod database;
mod entities;
mod handlers;
mod packet;
mod ranch;
mod server;
mod settings;

use tokio::{signal, sync::Mutex};

use std::{error::Error, fs::File, str::FromStr, sync::Arc};

use crate::{
    database::{Database, init_database},
    server::{Server, ServerType},
    settings::Settings,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = match File::open("settings.json") {
        Ok(file) => {
            // Load settings
            serde_json::from_reader(file)?
        }
        Err(_) => {
            // Generate settings and store them to file
            let settings = Settings::default();
            let file = File::options()
                .create(true)
                .write(true)
                .truncate(true)
                .open("settings.json")?;
            serde_json::to_writer_pretty(file, &settings)?;
            settings
        }
    };

    // Set up database
    println!("Setting up database");
    let (embedded_psql, connection_url) = init_database(&settings.database).await?;
    println!("Connected to database on {}", connection_url);

    let pg_config = tokio_postgres::Config::from_str(&connection_url)?;
    let database = Arc::new(Mutex::new(
        Database::new(&settings.database, pg_config).await?,
    ));

    // Set up servers.
    let lobby_server = if settings.lobby_server.enabled {
        Some(Server::new(ServerType::Lobby, &settings, Arc::clone(&database)).await?)
    } else {
        None
    };
    let ranch_server = if settings.ranch_server.enabled {
        Some(Server::new(ServerType::Ranch, &settings, Arc::clone(&database)).await?)
    } else {
        None
    };

    match signal::ctrl_c().await {
        Ok(()) => {
            println!("Received shutdown signal, stopping servers...");
        }
        Err(err) => {
            eprintln!(
                "Unable to listen for shutdown signal: {}. Shutting down",
                err
            );
            // we also shut down in case of error
        }
    }

    // TODO: Move these to Drop traits? Maybe not a good idea
    if let Some(lobby_server) = lobby_server {
        lobby_server.lock().await.stop().await?;
    }
    if let Some(ranch_server) = ranch_server {
        ranch_server.lock().await.stop().await?;
    }

    if let Some(embedded_psql) = embedded_psql {
        embedded_psql.stop().await?;
    }

    Ok(())
}

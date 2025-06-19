mod commands;
mod database;
mod entities;
mod handlers;
mod packet;
mod server;
mod settings;

use tokio::{signal, sync::Mutex};

use std::{error::Error, str::FromStr, sync::Arc};

use crate::{
    database::{Database, init_database},
    server::{Server, ServerType},
    settings::Settings,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Load from file
    let settings = Settings::default();

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

    match signal::ctrl_c().await {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        }
    }

    // TODO: Move these to Drop traits? Maybe not a good idea
    if let Some(lobby_server) = lobby_server {
        lobby_server.lock().await.stop().await?;
    }

    if let Some(embedded_psql) = embedded_psql {
        embedded_psql.stop().await?;
    }

    Ok(())
}

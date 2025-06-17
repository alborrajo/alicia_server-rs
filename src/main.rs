mod commands;
mod database;
mod entities;
mod handlers;
mod packet;
mod server;

use tokio::{signal, sync::Mutex};

use std::{error::Error, str::FromStr, sync::Arc};

use crate::{
    database::{Database, init_database},
    server::Server,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set up database
    println!("Setting up database");
    let (embedded_psql, connection_url, init_database) = init_database().await?;
    println!("Database running on {}", connection_url);

    let pg_config = tokio_postgres::Config::from_str(&connection_url)?;
    let database = Database::new(pg_config, init_database).await?;

    // Set up Lobby server.
    // TODO: Move address to config
    let addr = "0.0.0.0:10030";
    let server = Server::new("Lobby".into(), addr, Arc::new(Mutex::new(database))).await?;

    match signal::ctrl_c().await {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        }
    }

    server.lock().await.stop().await?;
    embedded_psql.stop().await?;
    Ok(())
}

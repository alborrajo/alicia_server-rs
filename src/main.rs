mod commands;
mod database;
mod entities;
mod handlers;
mod packet;
mod server;

use tokio::{signal, sync::Mutex};

use std::{error::Error, sync::Arc};

use crate::{database::Database, server::Server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set up database
    let database = Arc::new(Mutex::new(Database::new().await?));

    // Set up Lobby server.
    // TODO: Move address to config
    let addr = "0.0.0.0:10030";
    let server = Server::new("Lobby".into(), addr, database.clone()).await?;

    match signal::ctrl_c().await {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        }
    }

    server.lock().await.stop().await?;
    database.lock().await.stop().await?;
    Ok(())
}

use std::error::Error;

use postgresql_embedded::PostgreSQL;
use std::env;
use tokio::task::JoinHandle;
use tokio_postgres::{Client, NoTls};

const DATABASE_NAME: &str = "alicia";

pub struct Database {
    embedded_psql: PostgreSQL,
    psql_client: Option<Client>,
    psql_task: Option<JoinHandle<()>>,
}
impl Database {
    pub async fn new() -> Result<Database, Box<dyn Error>> {
        let mut embedded_psql = PostgreSQL::default();

        // embedded_psql.setup().await?;
        // embedded_psql.start().await?;

        // let database_exists = embedded_psql.database_exists(DATABASE_NAME).await?;
        // let wipe_on_startup = true; // TODO: Remove, make configurable, etc

        // let mut init_database = false;
        // if database_exists {
        //     if wipe_on_startup {
        //         embedded_psql.drop_database(DATABASE_NAME).await?;
        //         init_database = true;
        //     }
        // } else {
        //     init_database = true;
        // }

        // let (client, connection) =
        //     tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;
        // let psql_task = Some(tokio::spawn(async move {
        //     if let Err(e) = connection.await {
        //         eprintln!("Database connection error: {}", e);
        //     }
        // }));

        // if init_database {
        //     embedded_psql.create_database(DATABASE_NAME);
        //     let schema_path = env::current_exe()?
        //         .parent()
        //         .ok_or("Failed to get directory of current executable")?
        //         .join("res/schema.sql");
        //     let schema = tokio::fs::read_to_string(schema_path).await?;
        //     client.batch_execute(&schema).await?;
        // }

        Ok(Database {
            embedded_psql,
            psql_client: None, //Some(client),
            psql_task: None,   //psql_task,
        })
    }

    pub async fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        // self.embedded_psql.stop().await?;
        if let Some(psql_task) = &self.psql_task {
            psql_task.abort();
        }
        Ok(())
    }
}

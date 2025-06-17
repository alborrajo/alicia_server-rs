use std::{error::Error, path::PathBuf, time::Duration};

use postgresql_embedded::{PostgreSQL, Settings};
use tokio_postgres::{Config, NoTls, Transaction};

pub mod account;

const DATABASE_NAME: &str = "alicia";

pub async fn init_database() -> Result<(PostgreSQL, String, bool), Box<dyn Error>> {
    let settings = Settings {
        timeout: Some(Duration::from_secs(60)),
        ..Default::default()
    };
    let mut embedded_psql = PostgreSQL::new(settings);

    embedded_psql.setup().await?;
    embedded_psql.start().await?;

    let database_exists = embedded_psql.database_exists(DATABASE_NAME).await?;
    let wipe_on_startup = true; // TODO: Remove, make configurable, etc

    let mut init_database = false;
    if database_exists {
        if wipe_on_startup {
            embedded_psql.drop_database(DATABASE_NAME).await?;
            init_database = true;
        }
    } else {
        init_database = true;
    }

    if init_database {
        embedded_psql.create_database(DATABASE_NAME).await?;
    }

    let connection_string = embedded_psql.settings().url(DATABASE_NAME).to_owned();
    Ok((embedded_psql, connection_string, init_database))
}

pub struct Database {
    db_pool: deadpool_postgres::Pool,
}
impl Database {
    pub async fn new(pg_config: Config, init_database: bool) -> Result<Database, Box<dyn Error>> {
        let mgr_config = deadpool_postgres::ManagerConfig {
            recycling_method: deadpool_postgres::RecyclingMethod::Fast,
        };
        let mgr = deadpool_postgres::Manager::from_config(pg_config, NoTls, mgr_config);
        let db_pool = deadpool_postgres::Pool::builder(mgr)
            .max_size(16)
            .build()
            .unwrap();

        let client = db_pool.get().await?;

        if init_database {
            let schema_path = PathBuf::from("res/schema.sql");
            let schema = tokio::fs::read_to_string(schema_path).await?;
            client.batch_execute(&schema).await?;
        }

        Ok(Database { db_pool })
    }

    pub async fn run_in_transaction<T>(
        &mut self,
        mut function: impl AsyncFnMut(&mut Transaction) -> Result<T, Box<dyn Error>>,
    ) -> Result<T, Box<dyn Error>> {
        let mut psql_client = self.db_pool.get().await?;
        let mut transaction = psql_client.transaction().await?;
        let result = function(&mut transaction).await;
        if result.is_ok() {
            transaction.commit().await?;
        } else {
            transaction.rollback().await?;
        }
        result
    }
}

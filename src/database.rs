use std::{error::Error, ffi::CString, path::PathBuf, time::Duration};

use postgresql_embedded::{PostgreSQL, Settings};
use tokio_postgres::{
    Config, NoTls, Transaction,
    types::{FromSql, IsNull, ToSql, Type, accepts, private::BytesMut, to_sql_checked},
};

use crate::settings::DatabaseSettings;

pub mod account;
pub mod character;
pub mod horse;

pub async fn init_database(
    settings: &DatabaseSettings,
) -> Result<(Option<PostgreSQL>, String), Box<dyn Error>> {
    if let Some(url) = &settings.url {
        Ok((None, url.to_owned()))
    } else {
        let embedded_psql_settings = Settings {
            timeout: Some(Duration::from_secs(60)),
            ..Default::default()
        };
        let mut embedded_psql = PostgreSQL::new(embedded_psql_settings);

        embedded_psql.setup().await?;
        embedded_psql.start().await?;

        if !embedded_psql.database_exists(DATABASE_NAME).await? {
            embedded_psql.create_database(DATABASE_NAME).await?;
        }

        let url = embedded_psql.settings().url(DATABASE_NAME).to_owned();

        Ok((Some(embedded_psql), url))
    }
}

const DATABASE_NAME: &str = "alicia";

pub struct Database {
    db_pool: deadpool_postgres::Pool,
}
impl Database {
    pub async fn new(
        db_settings: &DatabaseSettings,
        pg_config: Config,
    ) -> Result<Database, Box<dyn Error>> {
        let mgr_config = deadpool_postgres::ManagerConfig {
            recycling_method: deadpool_postgres::RecyclingMethod::Fast,
        };
        let mgr = deadpool_postgres::Manager::from_config(pg_config, NoTls, mgr_config);
        let db_pool = deadpool_postgres::Pool::builder(mgr)
            .max_size(16)
            .build()
            .unwrap();

        let client = db_pool.get().await?;

        if db_settings.wipe_on_startup {
            // TODO: Wipe existing schema
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

// Wrapper to be able to insert CString into SQL
#[derive(Debug)]
pub struct CStringSql {
    pub value: CString,
}
impl<'a> FromSql<'a> for CStringSql {
    fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        Ok(CStringSql {
            value: CString::new(raw)?,
        })
    }
    accepts!(VARCHAR, TEXT);
}
impl ToSql for CStringSql {
    fn to_sql(
        &self,
        _: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        w.extend_from_slice(self.value.as_bytes());
        Ok(IsNull::No)
    }
    accepts!(VARCHAR, TEXT);
    to_sql_checked!();
}
impl From<CStringSql> for CString {
    fn from(wrapper: CStringSql) -> Self {
        wrapper.value
    }
}
impl From<CString> for CStringSql {
    fn from(value: CString) -> Self {
        CStringSql { value }
    }
}

// Wrappers to be able to insert unsigned types into SQL
macro_rules! define_unsigned_int_db_wrapper {
    ($name:ident, $innertype:ty, $dbtype:ty, $psqltype:ident, $bytecount:literal) => {
        #[derive(Debug)]
        pub struct $name {
            pub value: $innertype,
        }
        impl<'a> FromSql<'a> for $name {
            fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
                if raw.len() != $bytecount {
                    return Err(format!("expected to get {} bytes", $bytecount).into());
                }
                let db_value = <$dbtype>::from_be_bytes(raw.try_into()?);
                Ok($name {
                    value: db_value as $innertype,
                })
            }
            accepts!($psqltype);
        }
        impl ToSql for $name {
            fn to_sql(
                &self,
                _: &Type,
                w: &mut BytesMut,
            ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
                w.extend_from_slice(&(self.value as $dbtype).to_be_bytes());
                Ok(IsNull::No)
            }
            accepts!($psqltype);
            to_sql_checked!();
        }
        impl From<$name> for $innertype {
            fn from(wrapper: $name) -> Self {
                wrapper.value
            }
        }
        impl From<$innertype> for $name {
            fn from(value: $innertype) -> Self {
                $name { value }
            }
        }
    };
}

define_unsigned_int_db_wrapper!(U8Sql, u8, i16, INT2, 2);
define_unsigned_int_db_wrapper!(U16Sql, u16, i16, INT2, 2);
define_unsigned_int_db_wrapper!(U32Sql, u32, i32, INT4, 4);

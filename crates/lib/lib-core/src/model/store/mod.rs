// region:    --- Modules

mod error;

pub use self::error::{Error, Result};
use crate::core_config;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

// endregion: --- Modules

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
	let max_connections = if cfg!(test) { 1 } else { 5 };

    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&core_config().DB_URL)
        .await
        .map_err(|ex| Error::FailToCreatePool(ex.to_string()))
}

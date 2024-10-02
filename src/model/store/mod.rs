// region: --- Modules
mod error;
pub use self::error::{Error, Result};

use crate::config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
// endregion: --- Modules

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
    // * See NOTE 1) below
    let max_connections = if cfg!(test) { 1 } else { 5 };

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&config().DB_URL)
        .await
        .map_err(|e| Error::FailToCreatePool(e.to_string()))
}
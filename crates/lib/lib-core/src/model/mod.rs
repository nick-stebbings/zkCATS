// region:    --- Modules

mod base;
mod error;
mod store;

pub mod user;
pub mod community;

pub use self::error::{Error, Result};
use store::{Db, new_db_pool};

// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    // Expose to app
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;
        Ok(ModelManager { db })
    }

    // Expose to mode layer but not above
    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
}

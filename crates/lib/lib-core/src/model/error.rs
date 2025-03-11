use serde::Serialize;

use super::store;
use crate::crypt;

pub type Result<T> = core::result::Result<T, Error>;

use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    // -- Model
    EntityNotFound { entity: &'static str, id: i64 },
    // -- Modules
    ModelCrypt(crypt::Error),
    Store(store::Error),

    // -- Externals
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// region:    --- Froms

impl From<crypt::Error> for Error {
    fn from(val: crypt::Error) -> Self {
        Self::ModelCrypt(val)
    }
}

impl From<store::Error> for Error {
    fn from(val: store::Error) -> Self {
        Self::Store(val)
    }
}

impl From<sqlx::Error> for Error {
    fn from(val: sqlx::Error) -> Self {
        Self::Sqlx(val)
    }
}
// endregion: --- Froms

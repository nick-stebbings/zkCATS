use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_more::From;
use lib_core::model;
use serde::Serialize;
use std::sync::Arc;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;
pub type ClientError = Box<dyn std::error::Error>; // For early dev.

#[derive(Debug, Serialize, From, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- Login
    LoginFailUsernameNotFound,
    LoginFailUserHasNoPwd {
        user_id: i64,
    },
    LoginFailPasswordNotMatching {
        user_id: i64,
    },

    // -- Modules
    #[from]
    Model(model::Error),

    // -- CtxExtError
    #[from]
    CtxExt,
    // -- Extractors
    ReqStampNotInReqExt,
}

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{:<12} - model::Error {self:?}", "INTO_RES");

        // Create a placeholder Axum reponse.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the reponse.
        response.extensions_mut().insert(Arc::new(self));

        response
    }
}
// endregion: --- Axum IntoResponse

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

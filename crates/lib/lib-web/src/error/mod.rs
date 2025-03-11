use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_more::From;
use lib_core::crypt::{self, token};
use lib_core::model;
use serde::Serialize;
use std::sync::Arc;
use tracing::debug;

use crate::middleware::mw_auth::CtxExtError;

pub type Result<T> = core::result::Result<T, Error>;

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
    #[from]
    Token(token::Error),

    // -- CtxExtError
    #[from]
    CtxExt(CtxExtError),

    // -- Extractors
    ReqStampNotInReqExt,
}

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{:<12} - model::Error {self:?}", "INTO_RES");

        // Create a placeholder Axum reponse.
        let mut response = self.client_status_and_error().0.into_response();

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

// region:    --- ClientError

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use crate::error::Error::*;
        #[allow(unreachable_patterns)]
        match self {
            // -- Login
            LoginFailUsernameNotFound
            | LoginFailUserHasNoPwd { .. }
            | LoginFailPasswordNotMatching { .. } => {
                (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
            }
            // -- Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    SERVICE_ERROR,
}

// endregion: --- ClientError

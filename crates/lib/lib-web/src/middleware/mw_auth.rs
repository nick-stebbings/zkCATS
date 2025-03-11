use std::str::FromStr;

use crate::{
    Error,
    error::Result,
    token::{AUTH_TOKEN, set_token_cookie},
};
use axum::{
    body::Body,
    extract::{FromRequestParts, Request, State},
    http::request::Parts,
    middleware::Next,
    response::Response,
};
use lib_core::{
    config::core_config,
    crypt::token::{Token, generate_web_token, validate_web_token},
    ctx::Ctx,
    model::{
        ModelManager,
        user::{UserBmc, UserForAuth},
    },
};
use lib_util::time::now_utc_plus_sec_str;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

pub async fn mw_ctx_require(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
    debug!("{:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolve(
    mm: State<ModelManager>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    debug!("{:<12} - mw_ctx_resolve", "MIDDLEWARE");

    let ctx_ext_result = _ctx_resolve(mm, &cookies).await;

    if ctx_ext_result.is_err() && !matches!(ctx_ext_result, Err(CtxExtError::TokenNotInCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    // store the ctx_ext result in the request extension for Ctx extractor
    req.extensions_mut().insert(ctx_ext_result);

    Ok(next.run(req).await)
}

async fn _ctx_resolve(mm: State<ModelManager>, cookies: &Cookies) -> CtxExtResult {
    // Get token string
    let token = cookies
        .get(AUTH_TOKEN)
        .ok_or(CtxExtError::TokenNotInCookie)?
        .value()
        .to_owned();

    // Parse token
    let parsed_token: Token = Token::from_str(&token).map_err(|_| CtxExtError::TokenWrongFormat)?;

    // Get UserForAuth
    let username_for_auth = &parsed_token.ident.clone();
    let ctx = Ctx::root_ctx();
    let user_for_auth: UserForAuth = UserBmc::get_first_by_username(&ctx, &mm, &username_for_auth)
        .await
        .map_err(|ex| CtxExtError::ModelAccessError(ex.to_string()))?
        .ok_or(CtxExtError::UserNotFound)?;

    // Validate token
    validate_web_token(parsed_token, &user_for_auth.token_salt.to_string())
        .map_err(|_| CtxExtError::FailValidate)?;

    // Update Token
    set_token_cookie(
        cookies,
        username_for_auth,
        &user_for_auth.token_salt.to_string(),
    )
    .map_err(|_| CtxExtError::CannotSetTokenCookie)?;

    // Create CtxExtResult
    Ok(CtxW(Ctx::new(user_for_auth.id).map_err(|ex| {
        CtxExtError::CtxCreateFail(ex.to_string())
    })?))
}
// region:    --- Ctx Extractor

#[derive(Debug, Clone)]
pub struct CtxW(pub Ctx);

impl<S: Send + Sync> FromRequestParts<S> for CtxW {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("{:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<CtxExtResult>()
            .ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
            .clone()
            .map_err(Error::CtxExt)
    }
}
// endregion: --- Ctx Extractor

// region:    --- Ctx Extractor Result/Error
type CtxExtResult = core::result::Result<CtxW, CtxExtError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxExtError {
    TokenNotInCookie,
    TokenWrongFormat,

    UserNotFound,
    ModelAccessError(String),
    FailValidate,
    CannotSetTokenCookie,

    CtxNotInRequestExt,
    CtxCreateFail(String),
}
// endregion: --- Ctx Extractor Result/Error

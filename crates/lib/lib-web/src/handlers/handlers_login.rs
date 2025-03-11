use crate::{
    error::{Error, Result},
    token::set_token_cookie,
};
use axum::{Json, Router, extract::State, routing::post};
use lib_core::{
    crypt::{EncryptContent, password},
    ctx::Ctx,
    model::{
        ModelManager,
        user::{UserBmc, UserForLogin},
    },
};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::Cookies;
use tracing::debug;

pub async fn api_login_handler(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
    debug!("{:<12} - api_login_handler", "HANDLER");

    let LoginPayload {
        username,
        pwd: pwd_clear,
    } = payload;
    let root_ctx = Ctx::root_ctx();

    // -- Get the user
    let user: UserForLogin = UserBmc::get_first_by_username(&root_ctx, &mm, &username)
        .await?
        .ok_or(Error::LoginFailUsernameNotFound)?;

    // -- Check pw exists
    let Some(pwd) = user.pwd else {
        return Err(Error::LoginFailUserHasNoPwd { user_id: user.id });
    };

    password::validate_pwd(
        &EncryptContent {
            content: pwd_clear,
            salt: user.pwd_salt.to_string(),
        },
        &pwd,
    )
    .map_err(|_| Error::LoginFailPasswordNotMatching { user_id: user.id })?;

    // Set token cookie
    set_token_cookie(&cookies, &user.username, &user.token_salt.to_string())?;

    Ok(Json(json!({
        "success": true
    })))
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub pwd: String,
}

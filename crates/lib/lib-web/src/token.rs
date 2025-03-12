use crate::error::Result;
use lib_core::crypt::token::generate_web_token;
use tower_cookies::{Cookie, Cookies, cookie::SameSite};

pub const AUTH_TOKEN: &str = "auth-token";

pub fn set_token_cookie(cookies: &Cookies, user: &str, salt: &str) -> Result<()> {
    let token = generate_web_token(user, salt)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Strict);

    cookie.set_secure(true);

    cookie.set_path("/");

    cookies.add(cookie);

    Ok(())
}

pub fn remove_token_cookie(cookies: &Cookies) -> Result<()> {
    let mut cookie = Cookie::from(AUTH_TOKEN);
    cookie.set_path("/");

    cookies.remove(cookie);

    Ok(())
}

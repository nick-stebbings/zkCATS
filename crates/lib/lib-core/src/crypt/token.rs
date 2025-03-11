use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use lib_util::{
    b64::{b64_url_decode, b64_url_encode},
    time::{now_utc, now_utc_plus_sec_str, parse_utc},
};

use crate::{
    config::core_config,
    crypt::{EncryptContent, encrypt_into_b64u},
};

pub use super::error::{Error, Result};

// region:    --- Token Type

pub struct Token {
    pub ident: String,     // identifier,
    pub exp: String,       // Expiration date in Rfc3339
    pub sign_b64u: String, // signature for comparison
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            base64_url::encode(&self.ident),
            base64_url::encode(&self.exp),
            self.sign_b64u,
        )
    }
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(token_str: &str) -> Result<Token> {
        let splits = token_str.split(".").collect::<Vec<&str>>();

        if splits.len() != 3 {
            return Err(Error::TokenInvalidFormat);
        }
        let (ident_b64u, exp_b64u, sign_b64u) = (splits[0], splits[1], splits[2]);

        Ok(Self {
            ident: b64_url_decode(ident_b64u).map_err(|_| Error::TokenCannotDecodeIdent)?,
            exp: b64_url_decode(ident_b64u).map_err(|_| Error::TokenCannotDecodeExp)?,
            sign_b64u: sign_b64u.to_string(),
        })
    }
}

pub fn generate_web_token(user: &str, salt: &str) -> Result<Token> {
    let config = core_config();
    _generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)
}

pub fn validate_web_token(origin_token: Token, salt: &str) -> Result<()> {
    let config = core_config();
    _validate_token_sign_and_exp(origin_token, salt, &config.TOKEN_KEY)
}
// endregion: --- Token Type

// region:    --- (private) Token gen/validation

fn _generate_token(ident: &str, duration_sec: f64, salt: &str, key: &[u8]) -> Result<Token> {
    // Compute first two components
    let ident = ident.to_string();
    let exp = now_utc_plus_sec_str(duration_sec);

    // Sign first two components
    let sign_b64u = _token_sign_into_b64u(&ident, &exp, &salt, key)?;

    Ok(Token {
        ident,
        exp,
        sign_b64u,
    })
}

fn _validate_token_sign_and_exp(origin_token: Token, salt: &str, key: &[u8]) -> Result<()> {
    let Token {
        ident,
        exp,
        sign_b64u,
    } = origin_token;
    let config = core_config();

    let new_sign_b64u: String = _token_sign_into_b64u(&ident, &exp, &salt, key)?;
    if sign_b64u != new_sign_b64u {
        return Err(Error::TokenSignNotMatching);
    }

    let origin_exp_time = parse_utc(&exp).map_err(|_| Error::TokenExpNotIso)?;
    let now = now_utc();

    if now > origin_exp_time {
        return Err(Error::TokenExpired);
    }

    Ok(())
}

/// Create token signature from token parts/salt
fn _token_sign_into_b64u(ident: &str, exp: &str, salt: &str, key: &[u8]) -> Result<String> {
    let ident_part = b64_url_encode(ident);
    let exp_part = b64_url_encode(exp);

    let signed = encrypt_into_b64u(
        key,
        &EncryptContent {
            content: format!("{}.{}", ident_part, exp_part),
            salt: salt.into(),
        },
    )?;

    Ok(signed)
}

// endregion: --- Token gen/validation

// region:    --- Tests

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;
    use anyhow::Result;

    #[test]
    fn test_validate_web_token_ok() -> Result<()> {
        // S
        let fx_user = "my-username";
        let fx_salt = "my-salt";
        let fx_duration_secs = 0.02;
        let token_key = &core_config().TOKEN_KEY;
        let fx_token = _generate_token(fx_user, fx_duration_secs, fx_salt, token_key)?;

        // E
        thread::sleep(Duration::from_millis(10));
        let valid = validate_web_token(fx_token, fx_salt);

        // A
        valid?;

        Ok(())
    }

    #[test]
    fn test_validate_web_token_err_expired() -> Result<()> {
        // S
        let fx_user = "my-username";
        let fx_salt = "my-salt";
        let fx_duration_secs = 0.02;
        let token_key = &core_config().TOKEN_KEY;
        let fx_token = _generate_token(fx_user, fx_duration_secs, fx_salt, token_key)?;

        // E
        thread::sleep(Duration::from_millis(20));
        let invalid = validate_web_token(fx_token, fx_salt);

        // A
        assert!(
            matches!(invalid, Err(Error::TokenExpired)),
            "Should have matched Err(Error::TokenExpired) but was {invalid:?}"
        );

        Ok(())
    }
}

// endregion: --- Tests

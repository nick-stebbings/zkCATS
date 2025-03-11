
use std::{fmt::{Display, Formatter}, str::FromStr};

use lib_util::b64::b64_url_decode;

use crate::config::core_config;

use super::error::{Error, Result};

// region:    --- Token Type

pub struct Token {
  pub ident: String,    // identifier,
  pub exp: String,       // Expiration date in Rfc3339
  pub sign_b64u: String // signature for comparison
}

impl Display for Token {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    write!(f, "{}.{}.{}",
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
        return Err(Error::TokenInvalidFormat)
      }
      let (ident_b64u, exp_b64u, sign_b64u) = (splits[0], splits[1], splits[2]);
      
      return Ok(Self {
        ident: b64_url_decode(ident_b64u).map_err(|_| Error::TokenCannotDecodeIdent)?,
        exp: b64_url_decode(ident_b64u).map_err(|_| Error::TokenCannotDecodeExp)?,
        sign_b64u: sign_b64u.to_string()
      })
  }
}

fn generate_web_token(
  user: &str,
  salt: &str,
) -> Result<Token> {
  let config = core_config();
  _generate_token(user, config.TOKEN_DURATION_SEC, salt, &config.TOKEN_KEY)
}

fn validate_web_token(
  origin_token: Token,
  salt: &str,
) -> Result<()> {
  let config = core_config();
  _validate_token_sign_and_exp(origin_token, salt, &config.TOKEN_KEY)
}
// endregion: --- Token Type


// region:    --- (private) Token gen/validation

fn _generate_token(
  ident: &str,
  duration_sec: f64,
  salt: &str,
  key: &[u8]
) -> Result<Token> {
  todo!()

}

fn _validate_token_sign_and_exp(
  origin_token: Token,
  salt: &str,
  key: &[u8]
) -> Result<()> {

  Ok(())
}

/// Create token signature from token parts/salt
fn _token_sign_into_b64u(
  ident: &str,
  exp: &str,
  salt: &str,
  key: &[u8]
) -> Result<String> {
  todo!()
}

// endregion: --- Token gen/validation
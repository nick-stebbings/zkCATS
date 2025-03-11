use crate::{Error, Result};

pub fn b64_url_encode(content: &str) -> String {
  base64_url::encode(content)
}

pub fn b64_url_decode(b64u: &str) -> Result<String> {
  let decoded = base64_url::decode(b64u)
    .ok()
    .and_then(|r| String::from_utf8(r).ok())
    .ok_or(Error::FailToB64uDecode)?;

  Ok(decoded)
}
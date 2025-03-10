use crate::config::core_config;

use super::{EncryptContent, Error, Result, encrypt_into_b64u};

/// Encrypt password with the default scheme
pub fn encrypt_pwd(enc_content: &EncryptContent) -> Result<String> {
    let key = &core_config().PWD_KEY;

    let encrypted = encrypt_into_b64u(key, enc_content)?;

    Ok(format!("#01#{encrypted}"))
}

/// Validate password with the default scheme
pub fn validate_pwd(enc_content: &EncryptContent, pwd_ref: &str) -> Result<()> {
    let key = &core_config().PWD_KEY;

    let pwd = encrypt_pwd(enc_content)?;
    if (pwd != pwd_ref) {
        return Err(Error::PasswordInvalid);
    }
    Ok(())
}

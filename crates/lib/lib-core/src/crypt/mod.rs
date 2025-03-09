// region:    --- Modules

pub mod error;
pub mod password;
pub use error::{Error, Result};

use hmac::{Hmac, Mac};
use sha2::Sha256;

// endregion: --- Modules

pub struct EncryptContent {
    pub content: String, // Clear content.
    pub salt: String,    // Clear salt.
}

// We normalise into b64 url as it is portable and a reliable character set.
pub fn encrypt_into_b64u(key: &[u8], enc_content: &EncryptContent) -> Result<String> {
    let mut hmac = Hmac::<Sha256>::new_from_slice(key).map_err(|_| Error::KeyFailHmac)?;

    let EncryptContent { content, salt } = enc_content;

    hmac.update(content.as_bytes());
    hmac.update(salt.as_bytes());

    let result = hmac.finalize();

    let result_bytes = result.into_bytes();

    Ok(base64_url::encode(&result_bytes))
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use rand::RngCore;

    #[test]
    fn test_encrypt_into_b64u_ok() -> Result<()> {
        let mut fx_key = [0u8; 64]; // 512 bits = 64 bytes
        rand::rng().fill_bytes(&mut fx_key);

        let fx_enc_content = EncryptContent {
            content: "Hey there".to_string(),
            salt: "don't be salty".to_string(),
        };
        let result = encrypt_into_b64u(&fx_key, &fx_enc_content)?;

        let result2 = encrypt_into_b64u(&fx_key, &fx_enc_content)?;

        // Basic indempotency test
        assert_eq!(result, result2);

        Ok(())
    }
}

// endregion: --- Tests

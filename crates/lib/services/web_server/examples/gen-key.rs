pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // Ok for tools.

use rand::RngCore;

fn main() -> Result<()> {
    let mut key = [0u8; 64]; // 512 bits = 64 bytes
    rand::rng().fill_bytes(&mut key);
    println!("\nGenerated key from rand::thread_rng():\n{key:?}");

    let b64u = base64_url::encode(&key);
    println!("\nKey b64u encoded:\n{}", &b64u);

    Ok(())
}

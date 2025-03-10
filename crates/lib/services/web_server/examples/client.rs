// region:    --- Modules

use httpc_test::new_client;
use serde_json::json;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

// endregion: --- Modules

pub type Result<T> = core::result::Result<T, Error>;

pub type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let hc = new_client("http://localhost:8080")?;

    hc.do_get("/index.html").await?.print().await?;

    hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome",
        }),
    )
    .await?
    .print()
    .await?;

    Ok(())
}

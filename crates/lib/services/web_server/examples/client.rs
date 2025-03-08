// region:    --- Modules
// use config::web_config;

// use lib_web::middleware::mw_auth::{mw_ctx_require, mw_ctx_resolver};
// use lib_web::middleware::mw_req_stamp::mw_req_stamp_resolver;
// use lib_web::middleware::mw_res_map::mw_reponse_map;
// use lib_web::routes::routes_static;

// use crate::web::routes_login;

use httpc_test::new_client;
// use lib_core::_dev_utils;
// use lib_core::model::ModelManager;
// use tokio::net::TcpListener;
// use tower_cookies::CookieManagerLayer;
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

    Ok(())
}

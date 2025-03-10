// region:    --- Modules

pub mod config;
mod error;

pub use self::error::{Error, Result};
use lib_core::model::ModelManager;
use tokio::net::TcpListener;
// use config::web_config;

// use lib_web::middleware::mw_auth::{mw_ctx_require, mw_ctx_resolver};
// use lib_web::middleware::mw_req_stamp::mw_req_stamp_resolver;
// use lib_web::middleware::mw_res_map::mw_reponse_map;
// use lib_web::routes::routes_static;

// use crate::web::routes_login;

use axum::routing::get;
use axum::{Router, ServiceExt, middleware};
use httpc_test::new_client;
// use lib_core::_dev_utils;
// use lib_core::model::ModelManager;
// use tokio::net::TcpListener;
use crate::config::web_config;
use lib_core::_dev_util;
use lib_web::routes::{routes_login, routes_static};
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    _dev_util::init_dev().await;

    let mm: ModelManager = ModelManager::new().await?;

    let routes_all = Router::new()
        .merge(routes_login::routes(mm))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static::serve_dir(&web_config().WEB_FOLDER));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());

    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

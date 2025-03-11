// region:    --- Modules

pub mod config;
mod error;

pub use self::error::{Error, Result};
use crate::config::web_config;
use axum::response::Html;
use axum::routing::get;
use axum::{Router, ServiceExt, middleware};
use httpc_test::new_client;
use lib_core::_dev_util;
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_login;
use lib_web::middleware::mw_auth::{self, mw_ctx_resolve};
use lib_web::middleware::mw_res_map::mw_reponse_map;
use lib_web::routes::{routes_login, routes_static};
use tokio::net::TcpListener;
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

    let test_routes = Router::new()
        .route("/hello", get(|| async { Html("Hello world") }))
        .route_layer(middleware::from_fn(mw_auth::mw_ctx_require));

    let routes_all = Router::new()
        .merge(test_routes)
        .merge(routes_login::routes(mm.clone()))
        // .layer(middleware::map_response(mw_reponse_map))			.layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static::serve_dir(&web_config().WEB_FOLDER));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());

    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

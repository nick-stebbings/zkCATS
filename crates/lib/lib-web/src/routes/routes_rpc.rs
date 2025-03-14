use axum::{
    Json, Router,
    extract::State,
    response::{IntoResponse, Response},
    routing::post,
};
use lib_core::{ctx::Ctx, model::ModelManager};

use serde_json::Value;

use crate::{handlers::handlers_rpc, middleware::mw_auth::CtxW, rpc::RpcRequest};
pub fn routes(mm: ModelManager) -> Router {
    let rpc_router = rpc_router::Router::builder().append_resource(mm).build();

    // Build the Axum Router for '/rpc'
    Router::new()
        .route("/rpc", post(handlers_rpc::rpc_axum_handler))
        .with_state(rpc_router)
}

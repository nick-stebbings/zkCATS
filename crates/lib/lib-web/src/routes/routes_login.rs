use axum::{Router, routing::post};
use lib_core::model::ModelManager;

use crate::handlers::handlers_login::{api_login_handler, api_logoff_handler};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/login", post(api_login_handler))
        .route("/api/logoff", post(api_logoff_handler))
        .with_state(mm)
}

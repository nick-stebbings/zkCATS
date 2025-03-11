use crate::log::log_request;
use crate::middleware::mw_req_stamp::ReqStamp;
use axum::Json;
use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use lib_core::ctx::Ctx;
use lib_util::time::now_utc;
use serde_json::json;
use tracing::debug;
use uuid::Uuid;

pub async fn mw_reponse_map(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    debug!("{:<12} - mw_reponse_map", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    // -- Get the eventual response error.
    let web_error = res.extensions().get::<crate::Error>();
    let client_status_error = web_error.map(|se| se.client_status_and_error());

    // -- If client error, build the new reponse.
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            debug!("CLIENT ERROR BODY:\n{client_error_body}");

            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    // -- Build and log the server log line.
    let client_error = client_status_error.unzip().1;
    let req_stamp = ReqStamp {
        uuid,
        time_in: now_utc(),
    };
    let _ = crate::log::log_request(req_method, uri, req_stamp, ctx, web_error, client_error).await;

    debug!("\n");

    error_response.unwrap_or(res)
}

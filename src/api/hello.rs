use axum::{Router, http::StatusCode, routing::get};

use crate::api::AppState;

type HelloResponse = (StatusCode, String);
pub async fn hello_api() -> HelloResponse {
    (StatusCode::OK, "Hello world".to_string())
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(hello_api))
}

use axum::{
    Json,
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::{api::AppState, service::token::check_token};

#[derive(Serialize)]
struct AuthErrorResponse {
    success: bool,
    messages: String,
}

fn unauthorized(messages: &str) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        [(header::WWW_AUTHENTICATE, "Bearer")],
        Json(AuthErrorResponse {
            success: false,
            messages: messages.to_string(),
        }),
    )
        .into_response()
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let Some(token) = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
    else {
        return unauthorized("请先登录");
    };

    let claims = match check_token(token, &state.jwt_secret) {
        Ok(claims) => claims,
        Err(_) => return unauthorized("登录已失效，请重新登录"),
    };

    request.extensions_mut().insert(claims);

    next.run(request).await
}

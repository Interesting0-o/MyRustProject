use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

use crate::{
    SignUpRes,
    api::{AppState, with_sqlite_uow},
    librarian_sign_up, reader_sign_up,
    schema::{request::SignUpRequest, response::SignUpResponse},
};

type SignUpApiResponse = (StatusCode, Json<SignUpResponse>);

fn response(success: bool, messages: &str, status: StatusCode) -> SignUpApiResponse {
    (
        status,
        Json(SignUpResponse {
            success,
            messages: messages.to_string(),
            token: None,
        }),
    )
}

fn server_error() -> SignUpApiResponse {
    response(false, "服务器错误", StatusCode::FAILED_DEPENDENCY)
}

fn handle_result(result: Result<(), SignUpRes>) -> SignUpApiResponse {
    match result {
        Ok(()) => response(true, "注册成功", StatusCode::OK),
        Err(SignUpRes::AccountAlreadyExists) => response(false, "账号已被注册", StatusCode::OK),
        Err(SignUpRes::RepositoryError(_)) => server_error(),
    }
}

pub async fn reader_sign_up_api(
    State(state): State<AppState>,
    Json(req): Json<SignUpRequest>,
) -> SignUpApiResponse {
    with_sqlite_uow(state, |uow| {
        handle_result(reader_sign_up(uow, &req.name, &req.account, &req.password))
    })
    .unwrap_or_else(server_error)
}

pub async fn librarian_sign_up_api(
    State(state): State<AppState>,
    Json(req): Json<SignUpRequest>,
) -> SignUpApiResponse {
    with_sqlite_uow(state, |uow| {
        handle_result(librarian_sign_up(
            uow,
            &req.name,
            &req.account,
            &req.password,
        ))
    })
    .unwrap_or_else(server_error)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/sign_up/reader", post(reader_sign_up_api))
        .route("/sign_up/librarian", post(librarian_sign_up_api))
}

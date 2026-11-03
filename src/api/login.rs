use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

use crate::{
    LoginError,
    api::{AppState, with_sqlite_uow},
    librarian_login, reader_login,
    schema::{request::LoginRequest, response::LoginResponse},
};

type LoginApiResponse = (StatusCode, Json<LoginResponse>);

fn response(
    success: bool,
    messages: &str,
    token: Option<String>,
    name: Option<String>,
    status: StatusCode,
) -> LoginApiResponse {
    (
        status,
        Json(LoginResponse {
            success,
            messages: messages.to_string(),
            token,
            name,
        }),
    )
}

fn server_error() -> LoginApiResponse {
    response(
        false,
        "服务器错误",
        None,
        None,
        StatusCode::FAILED_DEPENDENCY,
    )
}

fn handle_result(result: Result<(String, String), LoginError>) -> LoginApiResponse {
    match result {
        Ok((account, name)) => response(
            true,
            "登录成功",
            Some(format!("FakeToken{account}")),
            Some(name),
            StatusCode::OK,
        ),
        Err(LoginError::RepositoryError(_)) => server_error(),
        Err(_) => response(false, "账号或密码错误", None, None, StatusCode::OK),
    }
}

pub async fn reader_login_api(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> LoginApiResponse {
    with_sqlite_uow(state, |uow| {
        handle_result(reader_login(uow, &req.account, &req.password).map(|r| (r.account, r.name)))
    })
    .unwrap_or_else(server_error)
}

pub async fn librarian_login_api(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> LoginApiResponse {
    with_sqlite_uow(state, |uow| {
        handle_result(
            librarian_login(uow, &req.account, &req.password).map(|l| (l.account, l.name)),
        )
    })
    .unwrap_or_else(server_error)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login/reader", post(reader_login_api))
        .route("/login/librarian", post(librarian_login_api))
}

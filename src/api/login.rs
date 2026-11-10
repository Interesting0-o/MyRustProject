use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

use crate::{
    LoginError,
    api::AppState,
    async_librarian_login, async_reader_login,
    schema::{request::LoginRequest, response::LoginResponse},
    service::token::create_token,
    with_mysql_uow,
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
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

fn handle_result(
    result: Result<(String, String), LoginError>,
    state: &AppState,
    role: &str,
) -> LoginApiResponse {
    match result {
        Ok((account, name)) => {
            match create_token(&account, role, &state.jwt_secret, state.jwt_expires_secs) {
                Ok(token) => response(true, "登录成功", Some(token), Some(name), StatusCode::OK),
                Err(_) => server_error(),
            }
        }
        Err(LoginError::RepositoryError(_)) => server_error(),
        Err(_) => response(
            false,
            "账号或密码错误",
            None,
            None,
            StatusCode::UNAUTHORIZED,
        ),
    }
}

pub async fn reader_login_api(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> LoginApiResponse {
    with_mysql_uow(&state.pool, async |uow| {
        handle_result(
            async_reader_login(uow, &req.account, &req.password)
                .await
                .map(|r| (r.account, r.name)),
            &state,
            "reader",
        )
    })
    .await
}

pub async fn librarian_login_api(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> LoginApiResponse {
    with_mysql_uow(&state.pool, async |uow| {
        handle_result(
            async_librarian_login(uow, &req.account, &req.password)
                .await
                .map(|l| (l.account, l.name)),
            &state,
            "librarian",
        )
    })
    .await
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login/reader", post(reader_login_api))
        .route("/login/librarian", post(librarian_login_api))
}

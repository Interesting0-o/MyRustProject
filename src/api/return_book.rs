use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

use crate::{
    ReturnBookError,
    api::AppState,
    async_return_book,
    schema::{request::ReturnBookRequest, response::ReturnBookResponse},
    with_mysql_uow,
};

type ReturnBookApiResponse = (StatusCode, Json<ReturnBookResponse>);

fn response(success: bool, messages: &str, status: StatusCode) -> ReturnBookApiResponse {
    (
        status,
        Json(ReturnBookResponse {
            success,
            messages: messages.to_string(),
        }),
    )
}

fn server_error() -> ReturnBookApiResponse {
    response(false, "服务器错误", StatusCode::INTERNAL_SERVER_ERROR)
}

fn handle_result(result: Result<(), ReturnBookError>) -> ReturnBookApiResponse {
    match result {
        Ok(()) => response(true, "还书成功", StatusCode::OK),
        Err(ReturnBookError::NonExistBID) => {
            response(false, "不存在的书籍号", StatusCode::NOT_FOUND)
        }
        Err(ReturnBookError::NotFoundRecord) => {
            response(false, "不存在的借阅记录", StatusCode::CONFLICT)
        }
        Err(_) => server_error(),
    }
}

pub async fn return_book_api(
    State(state): State<AppState>,
    Json(req): Json<ReturnBookRequest>,
) -> ReturnBookApiResponse {
    with_mysql_uow(&state.pool, async |uow| {
        handle_result(async_return_book(uow, &req.account, &req.bid).await)
    })
    .await
}

pub fn router() -> Router<AppState> {
    Router::new().route("/return_book", post(return_book_api))
}

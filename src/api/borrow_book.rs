use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

use crate::{
    BorrowBookError,
    api::{AppState, with_sqlite_uow},
    borrow_book,
    schema::{request::BorrowBookRequest, response::BorrowBookResponse},
};

type BorrowBookApiResponse = (StatusCode, Json<BorrowBookResponse>);

fn response(success: bool, messages: &str, status: StatusCode) -> BorrowBookApiResponse {
    (
        status,
        Json(BorrowBookResponse {
            success,
            messages: messages.to_string(),
        }),
    )
}

fn server_error() -> BorrowBookApiResponse {
    response(false, "服务器错误", StatusCode::FAILED_DEPENDENCY)
}

fn handle_result(result: Result<(), BorrowBookError>) -> BorrowBookApiResponse {
    match result {
        Ok(()) => response(true, "借书成功", StatusCode::OK),
        Err(BorrowBookError::NonExistBID) => response(false, "图书不存在", StatusCode::OK),
        Err(BorrowBookError::BookNumIsZero) => response(false, "图书已借完", StatusCode::OK),
        Err(_) => server_error(),
    }
}

pub async fn borrow_book_api(
    State(state): State<AppState>,
    Json(req): Json<BorrowBookRequest>,
) -> BorrowBookApiResponse {
    with_sqlite_uow(state, |uow| {
        handle_result(borrow_book(uow, &req.account, &req.bid))
    })
    .unwrap_or_else(server_error)
}

pub fn router() -> Router<AppState> {
    Router::new().route("/borrow_book", post(borrow_book_api))
}

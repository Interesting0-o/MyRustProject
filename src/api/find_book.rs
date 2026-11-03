use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

use crate::{
    Book, FindBookError, UnitOfWork,
    api::{AppState, with_sqlite_uow},
    schema::{
        request::{FindBookByBIDRequest, FindBookByNameRequest},
        response::FindBookResponse,
    },
};

type FindBookApiResponse = (StatusCode, Json<FindBookResponse>);

fn response(
    success: bool,
    messages: &str,
    books: Vec<Book>,
    status: StatusCode,
) -> FindBookApiResponse {
    (
        status,
        Json(FindBookResponse {
            success,
            messages: messages.to_string(),
            books,
        }),
    )
}

fn server_error() -> FindBookApiResponse {
    response(
        false,
        "服务器错误",
        Vec::new(),
        StatusCode::FAILED_DEPENDENCY,
    )
}

pub async fn find_book_by_bid_api(
    State(state): State<AppState>,
    Json(req): Json<FindBookByBIDRequest>,
) -> FindBookApiResponse {
    let bid = req.bid.trim();

    if bid.is_empty() {
        return response(false, "请输入图书编号", Vec::new(), StatusCode::BAD_REQUEST);
    }

    with_sqlite_uow(state, |uow| {
        let repos = uow.repositories();

        match repos.book().find_book_by_bid(bid) {
            Ok(book) => response(true, "查询成功", vec![book], StatusCode::OK),
            Err(FindBookError::NoResult) => {
                response(true, "未找到该编号的图书", Vec::new(), StatusCode::OK)
            }
            Err(FindBookError::Backend(_)) => server_error(),
        }
    })
    .unwrap_or_else(server_error)
}

pub async fn find_book_by_name_api(
    State(state): State<AppState>,
    Json(req): Json<FindBookByNameRequest>,
) -> FindBookApiResponse {
    let name = req.name.trim();

    if name.is_empty() {
        return response(false, "请输入书名", Vec::new(), StatusCode::BAD_REQUEST);
    }

    with_sqlite_uow(state, |uow| {
        let repos = uow.repositories();

        match repos.book().find_book_by_name(name) {
            Ok(books) => response(true, "查询成功", books, StatusCode::OK),
            Err(FindBookError::NoResult) => {
                response(true, "未找到相关图书", Vec::new(), StatusCode::OK)
            }
            Err(FindBookError::Backend(_)) => server_error(),
        }
    })
    .unwrap_or_else(server_error)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/find_book_by_bid", post(find_book_by_bid_api))
        .route("/find_book_by_name", post(find_book_by_name_api))
}

use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

use crate::{
    Book, FindBookError, UnitOfWork, UpdateBookError,
    api::{AppState, with_sqlite_uow},
    schema::{request::UpdateBookRequest, response::UpdateBookResponse},
};

type UpdateBookApiResponse = (StatusCode, Json<UpdateBookResponse>);

fn response(success: bool, messages: &str, code: StatusCode) -> UpdateBookApiResponse {
    (
        code,
        Json(UpdateBookResponse {
            success,
            messages: messages.to_string(),
        }),
    )
}
fn serve_error() -> UpdateBookApiResponse {
    response(false, "服务错误", StatusCode::FAILED_DEPENDENCY)
}
pub async fn update_book_name_api(
    State(state): State<AppState>,
    Json(req): Json<UpdateBookRequest>,
) -> UpdateBookApiResponse {
    with_sqlite_uow(state, |uow| {
        let repos = uow.repositories();
        let book_repo = repos.book();
        let book = match book_repo.find_book_by_bid(&req.bid) {
            Ok(b) => b,
            Err(FindBookError::NoResult) => {
                return response(false, "BID不存在", StatusCode::FAILED_DEPENDENCY);
            }
            Err(_) => return serve_error(),
        };
        if let Some(name) = req.name {
            match book_repo.update(Book { name, ..book }) {
                Ok(_) => response(true, "更新成功", StatusCode::OK),
                Err(UpdateBookError::NonExistedBID) => {
                    response(false, "BID不存在", StatusCode::FAILED_DEPENDENCY)
                }
                Err(_) => serve_error(),
            }
        } else {
            response(false, "参数不匹配", StatusCode::FAILED_DEPENDENCY)
        }
    })
    .unwrap_or_else(serve_error)
}

pub async fn update_book_price_api(
    State(state): State<AppState>,
    Json(req): Json<UpdateBookRequest>,
) -> UpdateBookApiResponse {
    with_sqlite_uow(state, |uow| {
        let repos = uow.repositories();
        let book_repo = repos.book();
        let book = match book_repo.find_book_by_bid(&req.bid) {
            Ok(b) => b,
            Err(FindBookError::NoResult) => {
                return response(false, "BID不存在", StatusCode::FAILED_DEPENDENCY);
            }
            Err(_) => return serve_error(),
        };
        if let Some(price) = req.price {
            match book_repo.update(Book { price, ..book }) {
                Ok(_) => response(true, "更新成功", StatusCode::OK),
                Err(UpdateBookError::NonExistedBID) => {
                    response(false, "BID不存在", StatusCode::FAILED_DEPENDENCY)
                }
                Err(_) => serve_error(),
            }
        } else {
            response(false, "参数不匹配", StatusCode::FAILED_DEPENDENCY)
        }
    })
    .unwrap_or_else(serve_error)
}

pub async fn update_book_num_api(
    State(state): State<AppState>,
    Json(req): Json<UpdateBookRequest>,
) -> UpdateBookApiResponse {
    with_sqlite_uow(state, |uow| {
        let repos = uow.repositories();
        let book_repo = repos.book();
        let book = match book_repo.find_book_by_bid(&req.bid) {
            Ok(b) => b,
            Err(FindBookError::NoResult) => {
                return response(false, "BID不存在", StatusCode::FAILED_DEPENDENCY);
            }
            Err(_) => return serve_error(),
        };
        if let Some(num) = req.num {
            match book_repo.update(Book { num, ..book }) {
                Ok(_) => response(true, "更新成功", StatusCode::OK),
                Err(UpdateBookError::NonExistedBID) => {
                    response(false, "BID不存在", StatusCode::FAILED_DEPENDENCY)
                }
                Err(_) => serve_error(),
            }
        } else {
            response(false, "参数不匹配", StatusCode::FAILED_DEPENDENCY)
        }
    })
    .unwrap_or_else(serve_error)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/update_book_name", post(update_book_name_api))
        .route("/update_book_price", post(update_book_price_api))
        .route("/update_book_num", post(update_book_num_api))
}

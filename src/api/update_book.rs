use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

use crate::{
    UpdateBookProfileError,
    api::AppState,
    async_update_book,
    schema::{request::UpdateBookRequest, response::UpdateBookResponse},
    with_mysql_uow,
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

fn server_error() -> UpdateBookApiResponse {
    response(false, "服务错误", StatusCode::INTERNAL_SERVER_ERROR)
}

fn handle_result(result: Result<(), UpdateBookProfileError>) -> UpdateBookApiResponse {
    match result {
        Ok(()) => response(true, "更新成功", StatusCode::OK),
        Err(UpdateBookProfileError::NonExistBID) => {
            response(false, "BID不存在", StatusCode::NOT_FOUND)
        }
        Err(UpdateBookProfileError::NoChanges) => {
            response(false, "没有需要更新的内容", StatusCode::OK)
        }
        Err(UpdateBookProfileError::RepositoryError(_)) => server_error(),
    }
}

pub async fn update_book_name_api(
    State(state): State<AppState>,
    Json(req): Json<UpdateBookRequest>,
) -> UpdateBookApiResponse {
    if req.name.is_none() {
        return response(false, "参数不匹配", StatusCode::BAD_REQUEST);
    }

    with_mysql_uow(&state.pool, async |uow| {
        handle_result(async_update_book(uow, &req.bid, req.name.as_deref(), None, None, None).await)
    })
    .await
}

pub async fn update_book_author_api(
    State(state): State<AppState>,
    Json(req): Json<UpdateBookRequest>,
) -> UpdateBookApiResponse {
    if req.author.is_none() {
        return response(false, "参数不匹配", StatusCode::BAD_REQUEST);
    }

    with_mysql_uow(&state.pool, async |uow| {
        handle_result(
            async_update_book(uow, &req.bid, None, req.author.as_deref(), None, None).await,
        )
    })
    .await
}

pub async fn update_book_price_api(
    State(state): State<AppState>,
    Json(req): Json<UpdateBookRequest>,
) -> UpdateBookApiResponse {
    if req.price.is_none() {
        return response(false, "参数不匹配", StatusCode::BAD_REQUEST);
    }

    with_mysql_uow(&state.pool, async |uow| {
        handle_result(async_update_book(uow, &req.bid, None, None, req.price, None).await)
    })
    .await
}

pub async fn update_book_num_api(
    State(state): State<AppState>,
    Json(req): Json<UpdateBookRequest>,
) -> UpdateBookApiResponse {
    if req.num.is_none() {
        return response(false, "参数不匹配", StatusCode::BAD_REQUEST);
    }

    with_mysql_uow(&state.pool, async |uow| {
        handle_result(async_update_book(uow, &req.bid, None, None, None, req.num).await)
    })
    .await
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/update_book_name", post(update_book_name_api))
        .route("/update_book_author", post(update_book_author_api))
        .route("/update_book_price", post(update_book_price_api))
        .route("/update_book_num", post(update_book_num_api))
}

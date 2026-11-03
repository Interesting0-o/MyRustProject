use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

use crate::{
    UpdateReaderProfileError,
    api::{AppState, with_sqlite_uow},
    schema::{request::UpdateReaderRequest, response::UpdateReaderResponse},
    update_reader,
};

type UpdateReaderApiResponse = (StatusCode, Json<UpdateReaderResponse>);

fn response(success: bool, messages: &str, status: StatusCode) -> UpdateReaderApiResponse {
    (
        status,
        Json(UpdateReaderResponse {
            success,
            messages: messages.to_string(),
        }),
    )
}

fn server_error() -> UpdateReaderApiResponse {
    response(false, "服务器错误", StatusCode::INTERNAL_SERVER_ERROR)
}

fn handle_result(result: Result<(), UpdateReaderProfileError>) -> UpdateReaderApiResponse {
    match result {
        Ok(()) => response(true, "更新成功", StatusCode::OK),
        Err(UpdateReaderProfileError::NonExistedAccount) => {
            response(false, "账号不存在", StatusCode::NOT_FOUND)
        }
        Err(UpdateReaderProfileError::PasswordError) => {
            response(false, "当前密码错误", StatusCode::UNAUTHORIZED)
        }
        Err(UpdateReaderProfileError::NoChanges) => {
            response(false, "没有需要更新的内容", StatusCode::OK)
        }
        Err(UpdateReaderProfileError::RepositoryError(_)) => server_error(),
    }
}

pub async fn update_reader_api(
    State(state): State<AppState>,
    Json(req): Json<UpdateReaderRequest>,
) -> UpdateReaderApiResponse {
    let account = req.account.trim();

    if account.is_empty() {
        return response(false, "请输入账号", StatusCode::BAD_REQUEST);
    }

    let name = match req.name.as_deref() {
        Some(raw) => {
            let name = raw.trim();
            if name.is_empty() {
                return response(false, "请输入姓名", StatusCode::BAD_REQUEST);
            }
            Some(name.to_string())
        }
        None => None,
    };

    let current_password = req.current_password.as_deref();
    let new_password = match req.new_password.as_deref() {
        Some("") => return response(false, "新密码不能为空", StatusCode::BAD_REQUEST),
        Some(password) => Some(password),
        None => None,
    };

    if name.is_none() && new_password.is_none() {
        return response(false, "参数不匹配", StatusCode::BAD_REQUEST);
    }

    with_sqlite_uow(state, |uow| {
        handle_result(update_reader(
            uow,
            account,
            name.as_deref(),
            current_password,
            new_password,
        ))
    })
    .unwrap_or_else(server_error)
}

pub fn router() -> Router<AppState> {
    Router::new().route("/update_reader", post(update_reader_api))
}

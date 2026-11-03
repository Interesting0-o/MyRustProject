use serde::Serialize;

use crate::Book;

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub messages: String,
    pub token: Option<String>,
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct SignUpResponse {
    pub success: bool,
    pub messages: String,
    pub token: Option<String>,
}
#[derive(Serialize)]
pub struct BorrowBookResponse {
    pub success: bool,
    pub messages: String,
}

#[derive(Serialize)]
pub struct ReturnBookResponse {
    pub success: bool,
    pub messages: String,
}

#[derive(Serialize)]
pub struct FindBookResponse {
    pub success: bool,
    pub messages: String,
    pub books: Vec<Book>,
}

#[derive(Serialize)]
pub struct UpdateBookResponse {
    pub success: bool,
    pub messages: String,
}

#[derive(Serialize)]
pub struct UpdateReaderResponse {
    pub success: bool,
    pub messages: String,
}

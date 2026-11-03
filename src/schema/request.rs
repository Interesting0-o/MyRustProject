use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub account: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SignUpRequest {
    pub name: String,
    pub account: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct BorrowBookRequest {
    pub bid: String,
    pub account: String,
}

#[derive(Deserialize)]
pub struct ReturnBookRequest {
    pub bid: String,
    pub account: String,
}

#[derive(Deserialize)]
pub struct FindBookByBIDRequest {
    pub bid: String,
}

#[derive(Deserialize)]
pub struct FindBookByNameRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateBookRequest {
    pub name: Option<String>,
    pub bid: String,
    pub price: Option<i64>,
    pub num: Option<u32>,
}

#[derive(Deserialize)]
pub struct UpdateReaderRequest {
    pub account: String,
    pub name: Option<String>,
    pub current_password: Option<String>,
    pub new_password: Option<String>,
}

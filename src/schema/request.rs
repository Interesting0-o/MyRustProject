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

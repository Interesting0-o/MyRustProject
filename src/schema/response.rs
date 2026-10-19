use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub messages: String,
    pub token: Option<String>,
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

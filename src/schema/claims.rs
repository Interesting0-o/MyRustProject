use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // 所属用户
    pub role: String, // 角色
    pub exp: u64,     // 过期时间
    pub iat: u64,     // 签发时间
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Book {
    pub author: String,
    pub bid: String,
    pub name: String,
    pub price: i64,
    pub num: u32,
}

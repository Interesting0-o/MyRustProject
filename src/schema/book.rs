use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct Book {
    pub author: String,
    pub bid: String,
    pub name: String,
    pub price: i64,
    pub num: u32,
}

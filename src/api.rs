pub mod borrow_book;
pub mod find_book;
pub mod login;
pub mod return_book;
pub mod sign_up;
pub mod update_book;
pub mod update_reader;

use sqlx::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
}

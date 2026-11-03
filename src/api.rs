pub mod borrow_book;
pub mod find_book;
pub mod login;
pub mod return_book;
pub mod sign_up;
pub mod update_book;
pub mod update_reader;

#[derive(Clone)]
pub struct AppState {
    pub db_path: String,
}

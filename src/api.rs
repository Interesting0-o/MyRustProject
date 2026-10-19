use rusqlite::Connection;

use crate::SqliteUnitOfWork;

pub mod borrow_book;
pub mod login;
pub mod return_book;
pub mod sign_up;

#[derive(Clone)]
pub struct AppState {
    pub db_path: String,
}

pub(crate) fn with_sqlite_uow<T, F>(state: AppState, f: F) -> Option<T>
where
    F: FnOnce(&mut SqliteUnitOfWork) -> T,
{
    let conn = Connection::open(state.db_path).ok()?;
    let mut uow = SqliteUnitOfWork::new(conn);
    Some(f(&mut uow))
}

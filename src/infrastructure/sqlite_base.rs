pub mod sqlite_base_book_repo;
pub mod sqlite_base_borrow_record_repo;
pub mod sqlite_base_librarian_repo;
pub mod sqlite_base_reader_repo;

pub enum SqliteBaseNewError {
    PathIsEmpty,
    ConnectError(rusqlite::Error),
}

use rusqlite::{Connection, Transaction};

pub mod sqlite_base_book_repo;
pub mod sqlite_base_borrow_record_repo;
pub mod sqlite_base_librarian_repo;
pub mod sqlite_base_reader_repo;

pub enum SqliteBaseNewError {
    PathIsEmpty,
    ConnectError(rusqlite::Error),
}

pub trait ForSqlRepo {
    fn conn(&self) -> &Connection;
}

impl ForSqlRepo for Connection {
    fn conn(&self) -> &Connection {
        self
    }
}

impl ForSqlRepo for Transaction<'_> {
    fn conn(&self) -> &Connection {
        self
    }
}

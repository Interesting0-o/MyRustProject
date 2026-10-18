use rusqlite::Connection;

use crate::{
    BookRepository, BorrowRecordRepository, LibrarianRepository, ReaderRepository, RepositoryGroup,
    SqliteBaseBookRepository, SqliteBaseBorrowRecordRepository, SqliteBaseLibrarianRepository,
    SqliteBaseReaderRepository, UnitOfWork, UnitOfWorkError,
};
pub mod sqlite_base_book_repo;
pub mod sqlite_base_borrow_record_repo;
pub mod sqlite_base_librarian_repo;
pub mod sqlite_base_reader_repo;

pub enum SqliteBaseNewError {
    PathIsEmpty,
    ConnectError(rusqlite::Error),
}

pub struct SqliteUnitOfWork {
    conn: Connection,
}

impl SqliteUnitOfWork {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }
}

/// 打开一个 sqlite 数据库连接并初始化全部表，供 [`SqliteUnitOfWork`] 使用。
pub fn new_sqlite_unit_of_work(path: &str) -> Result<SqliteUnitOfWork, SqliteBaseNewError> {
    if path.trim().is_empty() {
        return Err(SqliteBaseNewError::PathIsEmpty);
    }
    let conn = Connection::open(path).map_err(SqliteBaseNewError::ConnectError)?;
    sqlite_base_book_repo::create_book_table(&conn)?;
    sqlite_base_reader_repo::create_reader_table(&conn)?;
    sqlite_base_librarian_repo::create_librarian_table(&conn)?;
    sqlite_base_borrow_record_repo::create_borrow_record_table(&conn)?;
    Ok(SqliteUnitOfWork::new(conn))
}

impl UnitOfWork for SqliteUnitOfWork {
    fn repositories(&self) -> Box<dyn RepositoryGroup + '_> {
        Box::new(SqliteRepositoryGroup::new(&self.conn))
    }

    fn transaction<T, E>(
        &mut self,
        f: impl FnOnce(&dyn RepositoryGroup) -> Result<T, E>,
    ) -> Result<T, UnitOfWorkError<E>> {
        let tx = self
            .conn
            .transaction()
            .map_err(|e| UnitOfWorkError::Backend(Box::new(e)))?;
        let res = {
            let repo_group = SqliteRepositoryGroup::new(&tx);
            f(&repo_group).map_err(UnitOfWorkError::Transaction)?
        };
        tx.commit()
            .map_err(|e| UnitOfWorkError::Backend(Box::new(e)))?;
        Ok(res)
    }
}

pub struct SqliteRepositoryGroup<'a> {
    conn: &'a Connection,
}

impl<'a> SqliteRepositoryGroup<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }
}

impl RepositoryGroup for SqliteRepositoryGroup<'_> {
    fn book(&self) -> Box<dyn BookRepository + '_> {
        Box::new(SqliteBaseBookRepository::new(self.conn))
    }

    fn borrow_record(&self) -> Box<dyn BorrowRecordRepository + '_> {
        Box::new(SqliteBaseBorrowRecordRepository::new(self.conn))
    }

    fn reader(&self) -> Box<dyn ReaderRepository + '_> {
        Box::new(SqliteBaseReaderRepository::new(self.conn))
    }

    fn librarian(&self) -> Box<dyn LibrarianRepository + '_> {
        Box::new(SqliteBaseLibrarianRepository::new(self.conn))
    }
}

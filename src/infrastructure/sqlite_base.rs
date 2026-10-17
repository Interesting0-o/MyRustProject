use rusqlite::{Connection, Transaction};

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

pub struct SqliteRepositoryGroup<'a, T>
where
    T: ForSqlRepo,
{
    tx: &'a T,
}

impl<'a, T> SqliteRepositoryGroup<'a, T>
where
    T: ForSqlRepo,
{
    pub fn new(tx: &'a T) -> Self {
        Self { tx }
    }
}

impl<T> RepositoryGroup for SqliteRepositoryGroup<'_, T>
where
    T: ForSqlRepo,
{
    fn book(&self) -> Box<dyn BookRepository + '_> {
        Box::new(SqliteBaseBookRepository::new(self.tx))
    }

    fn borrow_record(&self) -> Box<dyn BorrowRecordRepository + '_> {
        Box::new(SqliteBaseBorrowRecordRepository::new(self.tx))
    }

    fn reader(&self) -> Box<dyn ReaderRepository + '_> {
        Box::new(SqliteBaseReaderRepository::new(self.tx))
    }

    fn librarian(&self) -> Box<dyn LibrarianRepository + '_> {
        Box::new(SqliteBaseLibrarianRepository::new(self.tx))
    }
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

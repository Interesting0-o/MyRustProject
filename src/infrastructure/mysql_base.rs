pub mod mysql_base_book_repo;
pub mod mysql_base_reader_repo;
use std::{future::Future, pin::Pin};

use crate::{
    AsyncBookRepository, AsyncBorrowRecordRepository, AsyncLibrarianRepository,
    AsyncReaderRepository, AsyncRepositoryGroup, AsyncUnitOfWork, UnitOfWorkError,
};
use mysql_base_book_repo::MySqlBaseBookRepository;
use mysql_base_reader_repo::MySqlBaseReaderRepository;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};

pub struct MySqlUnitOfWork {
    pool: MySqlPool,
}

#[derive(Debug)]
pub enum MySqlUNewError {
    URLIsEmpty,
    ConnectError,
}

impl MySqlUnitOfWork {
    fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AsyncUnitOfWork for MySqlUnitOfWork {
    fn repositories(&self) -> Box<dyn AsyncRepositoryGroup + '_> {
        Box::new(MySqlRepositoryGroup { pool: &self.pool })
    }

    async fn transaction<T, E, F>(&mut self, _f: F) -> Result<T, UnitOfWorkError<E>>
    where
        T: Send,
        E: Send,
        F: for<'a> FnOnce(
                &'a dyn AsyncRepositoryGroup,
            ) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'a>>
            + Send,
    {
        todo!();
    }
}

async fn new_mysql_uow(url: &str) -> Result<MySqlUnitOfWork, MySqlUNewError> {
    if url.trim().is_empty() {
        return Err(MySqlUNewError::URLIsEmpty);
    }

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(url)
        .await
        .map_err(|_| MySqlUNewError::ConnectError)?;
    Ok(MySqlUnitOfWork { pool })
}

/// 创建 MySQL 工作单元并交给 `f` 使用。
///
/// 连接串为空或连接失败时返回 [`MySqlUNewError`]。
pub async fn with_mysql_uow<T, F>(url: &str, f: F) -> Result<T, MySqlUNewError>
where
    F: AsyncFnOnce(&mut MySqlUnitOfWork) -> T,
{
    let mut uow = new_mysql_uow(url).await?;
    Ok(f(&mut uow).await)
}

// 仓库群
struct MySqlRepositoryGroup<'a> {
    pool: &'a MySqlPool,
}

impl<'a> AsyncRepositoryGroup for MySqlRepositoryGroup<'a> {
    fn book(&self) -> Box<dyn AsyncBookRepository + '_> {
        Box::new(MySqlBaseBookRepository::new(self.pool))
    }
    fn reader(&self) -> Box<dyn AsyncReaderRepository + '_> {
        Box::new(MySqlBaseReaderRepository::new(self.pool))
    }

    fn librarian(&self) -> Box<dyn AsyncLibrarianRepository + '_> {
        todo!();
    }
    fn borrow_record(&self) -> Box<dyn AsyncBorrowRecordRepository + '_> {
        todo!();
    }
}

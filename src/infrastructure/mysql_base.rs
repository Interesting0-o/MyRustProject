pub mod mysql_base_book_repo;
pub mod mysql_base_borrow_record_repo;
pub mod mysql_base_librarian_repo;
pub mod mysql_base_reader_repo;
use std::{future::Future, ops::DerefMut, pin::Pin};

use crate::{
    AsyncBookRepository, AsyncBorrowRecordRepository, AsyncLibrarianRepository,
    AsyncReaderRepository, AsyncRepositoryGroup, AsyncUnitOfWork, BackendError, UnitOfWorkError,
    infrastructure::mysql_base::mysql_base_librarian_repo::MySqlBaseLibrarianRepository,
};
use mysql_base_book_repo::MySqlBaseBookRepository;
use mysql_base_borrow_record_repo::MySqlBaseBorrowRecordRepository;
use mysql_base_reader_repo::MySqlBaseReaderRepository;
use sqlx::{
    MySqlPool,
    mysql::{MySqlConnection, MySqlPoolOptions},
};

pub async fn run_migrations(pool: &MySqlPool) -> Result<(), BackendError> {
    // sqlx 内部用 GET_LOCK(?, -1) 加锁，MariaDB 会返回 NULL 导致锁失效，
    // 这里显式加锁，保证多实例/并发启动时迁移不会重复执行。
    let mut lock_conn = pool
        .acquire()
        .await
        .map_err(|e| Box::new(e) as BackendError)?;
    let acquired: Option<i64> = sqlx::query_scalar("SELECT GET_LOCK(?, 60)")
        .bind("book_manager_migrations")
        .fetch_one(&mut *lock_conn)
        .await
        .map_err(|e| Box::new(e) as BackendError)?;

    if acquired != Some(1) {
        return Err(Box::new(std::io::Error::other("获取数据库迁移锁超时")));
    }

    let result = sqlx::migrate!("./migrations").run(pool).await;

    let _ = sqlx::query_scalar::<_, Option<i64>>("SELECT RELEASE_LOCK(?)")
        .bind("book_manager_migrations")
        .fetch_one(&mut *lock_conn)
        .await;

    result.map_err(|e| Box::new(e) as BackendError)
}

#[derive(Debug)]
pub enum MySqlUNewError {
    URLIsEmpty,
    ConnectError,
}

// 工作单元模式
pub struct MySqlUnitOfWork {
    pool: MySqlPool,
}

impl MySqlUnitOfWork {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AsyncUnitOfWork for MySqlUnitOfWork {
    async fn repositories(&mut self) -> Result<Box<dyn AsyncRepositoryGroup + '_>, BackendError> {
        let conn = self
            .pool
            .acquire()
            .await
            .map_err(|e| Box::new(e) as BackendError)?;
        Ok(Box::new(MySqlRepositoryGroup { conn }))
    }

    async fn transaction<T, E, F>(&mut self, f: F) -> Result<T, UnitOfWorkError<E>>
    where
        T: Send,
        E: Send,
        F: for<'a> FnOnce(
                &'a mut dyn AsyncRepositoryGroup,
            ) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'a>>
            + Send,
    {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| UnitOfWorkError::Backend(Box::new(e)))?;

        let result = {
            let mut repos = MySqlRepositoryGroup { conn: &mut *tx };
            f(&mut repos).await
        };

        match result {
            Ok(value) => {
                tx.commit()
                    .await
                    .map_err(|e| UnitOfWorkError::Backend(Box::new(e)))?;
                Ok(value)
            }
            Err(e) => {
                let _ = tx.rollback().await;
                Err(UnitOfWorkError::Transaction(e))
            }
        }
    }
}

pub async fn new_mysql_uow(url: &str) -> Result<MySqlUnitOfWork, MySqlUNewError> {
    if url.trim().is_empty() {
        return Err(MySqlUNewError::URLIsEmpty);
    }

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(url)
        .await
        .map_err(|_| MySqlUNewError::ConnectError)?;
    Ok(MySqlUnitOfWork::new(pool))
}

/// 使用已有连接池创建 MySQL 工作单元并交给 `f` 使用。
pub async fn with_mysql_uow<T, F>(pool: &MySqlPool, f: F) -> T
where
    F: AsyncFnOnce(&mut MySqlUnitOfWork) -> T,
{
    let mut uow = MySqlUnitOfWork::new(pool.clone());
    f(&mut uow).await
}

// 仓库群
struct MySqlRepositoryGroup<C> {
    conn: C,
}

impl<C> AsyncRepositoryGroup for MySqlRepositoryGroup<C>
where
    C: DerefMut<Target = MySqlConnection> + Send,
{
    fn book(&mut self) -> Box<dyn AsyncBookRepository + '_> {
        Box::new(MySqlBaseBookRepository::new(&mut self.conn))
    }
    fn reader(&mut self) -> Box<dyn AsyncReaderRepository + '_> {
        Box::new(MySqlBaseReaderRepository::new(&mut self.conn))
    }

    fn librarian(&mut self) -> Box<dyn AsyncLibrarianRepository + '_> {
        Box::new(MySqlBaseLibrarianRepository::new(&mut self.conn))
    }
    fn borrow_record(&mut self) -> Box<dyn AsyncBorrowRecordRepository + '_> {
        Box::new(MySqlBaseBorrowRecordRepository::new(&mut self.conn))
    }
}

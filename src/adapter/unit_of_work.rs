//! 事务（Unit of Work）抽象。
//!
//! service 只依赖这里的 trait，不关心底层是 sqlite、文件还是内存。

use std::{future::Future, pin::Pin};

use super::{
    BackendError,
    book_repo::{AsyncBookRepository, BookRepository},
    borrow_record_repo::{AsyncBorrowRecordRepository, BorrowRecordRepository},
    librarian_repo::{AsyncLibrarianRepository, LibrarianRepository},
    reader_repo::{AsyncReaderRepository, ReaderRepository},
};

/// 一次事务内可访问的仓储集合。
pub trait RepositoryGroup {
    /// 图书仓储。
    fn book(&self) -> Box<dyn BookRepository + '_>;

    /// 借阅记录仓储。
    fn borrow_record(&self) -> Box<dyn BorrowRecordRepository + '_>;

    /// 读者仓储。
    fn reader(&self) -> Box<dyn ReaderRepository + '_>;

    /// 管理员仓储。
    fn librarian(&self) -> Box<dyn LibrarianRepository + '_>;
}

/// 异步仓储集合，供 sqlx 等异步后端实现。
///
/// 与 [`RepositoryGroup`] 一一对应，区别是返回异步仓储契约。
pub trait AsyncRepositoryGroup: Send {
    /// 图书仓储。
    fn book(&mut self) -> Box<dyn AsyncBookRepository + '_>;

    /// 借阅记录仓储。
    fn borrow_record(&mut self) -> Box<dyn AsyncBorrowRecordRepository + '_>;

    /// 读者仓储。
    fn reader(&mut self) -> Box<dyn AsyncReaderRepository + '_>;

    /// 管理员仓储。
    fn librarian(&mut self) -> Box<dyn AsyncLibrarianRepository + '_>;
}

/// 事务失败的原因。
#[derive(Debug)]
pub enum UnitOfWorkError<E> {
    /// 开启或提交事务时出错。
    Backend(BackendError),
    /// 事务闭包返回的业务错误。
    Transaction(E),
}

/// 事务边界：闭包内的操作要么一起提交，要么一起回滚。
pub trait UnitOfWork {
    /// 获取只读仓储集合，适用于登录、注册、查询等自动提交场景。
    fn repositories(&self) -> Box<dyn RepositoryGroup + '_>;

    /// 在一个事务中执行 `f`。
    fn transaction<T, E>(
        &mut self,
        f: impl FnOnce(&dyn RepositoryGroup) -> Result<T, E>,
    ) -> Result<T, UnitOfWorkError<E>>;
}

/// 异步事务边界，与 [`UnitOfWork`] 对应，供 sqlx 等异步后端实现。
#[async_trait::async_trait]
pub trait AsyncUnitOfWork: Send {
    /// 获取只读仓储集合，适用于登录、注册、查询等自动提交场景。
    async fn repositories(&mut self) -> Result<Box<dyn AsyncRepositoryGroup + '_>, BackendError>;

    /// 在一个事务中执行 `f`。
    async fn transaction<T, E, F>(&mut self, f: F) -> Result<T, UnitOfWorkError<E>>
    where
        T: Send,
        E: Send,
        F: for<'a> FnOnce(
                &'a mut dyn AsyncRepositoryGroup,
            ) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'a>>
            + Send;
}

//! 事务（Unit of Work）抽象。
//!
//! service 只依赖这里的 trait，不关心底层是 sqlite、文件还是内存。

use super::{
    BackendError, book_repo::BookRepository, borrow_record_repo::BorrowRecordRepository,
    librarian_repo::LibrarianRepository, reader_repo::ReaderRepository,
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

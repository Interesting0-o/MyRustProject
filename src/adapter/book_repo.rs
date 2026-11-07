//! 图书仓储接口。

use crate::{Book, adapter::BackendError};

/// 新增图书时可能出现的错误。
#[derive(Debug)]
pub enum AddBookError {
    /// 业务规则：`book.bid` 已存在。
    BIDAlreadyExists,
    /// 仓储故障（IO、序列化、锁中毒等）。
    Backend(BackendError),
}

/// 更新图书时可能出现的错误。
#[derive(Debug)]
pub enum UpdateBookError {
    /// 业务规则：`book.bid` 不存在，更新无对象。
    NonExistedBID,
    /// 仓储故障（IO、序列化、锁中毒等）。
    Backend(BackendError),
}

/// 查询图书时可能出现的错误。
///
/// 「没查到」不算仓储故障，用 [`FindBookError::NoResult`] 表达空结果；
/// [`FindBookError::Backend`] 只表示仓储访问失败，让调用方能把「故障」和「空结果」分开处理。
#[derive(Debug)]
pub enum FindBookError {
    /// 空结果：按 `bid` 查不到对应图书。
    NoResult,
    /// 仓储故障（IO、序列化、锁中毒等）。
    Backend(BackendError),
}

/// 图书仓储的异步契约，供 sqlx 等异步后端实现。
#[async_trait::async_trait]
pub trait AsyncBookRepository: Send {
    /// 新增一本图书。
    ///
    /// # 错误
    ///
    /// 当 `book.bid` 已存在时返回 [`AddBookError::BIDAlreadyExists`]；
    /// 仓储访问失败时返回 [`AddBookError::Backend`]。
    async fn add(&mut self, book: Book) -> Result<(), AddBookError>;

    /// 按书名查找，返回所有同名图书；没有匹配时返回 [`FindBookError::NoResult`]。
    async fn find_book_by_name(&mut self, name: &str) -> Result<Vec<Book>, FindBookError>;

    /// 按编号 `bid` 精确查找，不存在时返回 [`FindBookError::NoResult`]。
    async fn find_book_by_bid(&mut self, bid: &str) -> Result<Book, FindBookError>;

    /// 更新图书信息，以 `book.bid` 作为定位依据。
    ///
    /// # 错误
    ///
    /// 当 `bid` 不存在时返回 [`UpdateBookError::NonExistedBID`]；
    /// 仓储访问失败时返回 [`UpdateBookError::Backend`]。
    async fn update(&mut self, book: Book) -> Result<(), UpdateBookError>;
}

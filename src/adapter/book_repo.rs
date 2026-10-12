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

/// 图书仓储：定义 [`Book`] 的读写操作。
///
/// 所有实现都应保证 `bid` 在仓储内唯一。
///
/// 写操作（[`add`](Self::add) / [`update`](Self::update)）同样只借用 `&self`。
/// 契约不规定仓储如何同步，由实现自行决定：内存实现用内部可变性，
/// 持久化实现依赖连接池/事务，两者都不需要调用方交出独占借用。
/// 这样契约就不会把「单线程独占」这个内存实现的特性，强加给所有后端。
///
/// 构造也不属于本契约：各实现的构造方式不同（可能失败、可能需要连接串、
/// 可能需要加载文件），由实现自行提供，调用方按具体类型构造。
pub trait BookRepository {
    /// 新增一本图书。
    ///
    /// # 错误
    ///
    /// 当 `book.bid` 已存在时返回 [`AddBookError::BIDAlreadyExists`]；
    /// 仓储访问失败时返回 [`AddBookError::Backend`]。
    fn add(&self, book: Book) -> Result<(), AddBookError>;

    /// 按书名查找，返回所有同名图书；没有匹配时返回 [`FindBookError::NoResult`]。
    ///
    /// # 错误
    ///
    /// 仓储访问失败时返回 [`FindBookError::Backend`]。
    fn find_book_by_name(&self, name: &str) -> Result<Vec<Book>, FindBookError>;

    /// 按编号 `bid` 精确查找，不存在时返回 [`FindBookError::NoResult`]。
    ///
    /// # 错误
    ///
    /// 仓储访问失败时返回 [`FindBookError::Backend`]。
    fn find_book_by_bid(&self, bid: &str) -> Result<Book, FindBookError>;

    /// 更新图书信息，以 `book.bid` 作为定位依据。
    ///
    /// `bid` 是主键，不会被这次更新改动：传入的 `bid` 只用于定位，
    /// 需与原记录保持一致。
    ///
    /// # 错误
    ///
    /// 当 `bid` 不存在时返回 [`UpdateBookError::NonExistedBID`]；
    /// 仓储访问失败时返回 [`UpdateBookError::Backend`]。
    fn update(&self, book: Book) -> Result<(), UpdateBookError>;
}

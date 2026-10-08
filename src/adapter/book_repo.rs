//! 图书仓储接口。

use crate::Book;

pub enum AddBookError {
    BIDAlreadyExists,
    /// 持久化到文件失败（磁盘满、权限不足等）。
    IoError(std::io::Error),
    RepositoryLockError,
}

#[derive(Debug)]
pub enum UpdateBookError {
    NonExistedBID,
    /// 持久化到文件失败（磁盘满、权限不足等）。
    IoError(std::io::Error),
    RepositoryLockError,
}

/// 查询图书时可能出现的错误。
///
/// 「没查到」不算错误：按 `bid` 查不到是 `Ok(None)`，按书名查不到是 `Ok(vec![])`。
/// 本枚举只表示仓储本身访问失败，让调用方能把「故障」和「空结果」分开处理。
#[derive(Debug)]
pub enum FindBookError {
    /// 读取持久化数据失败（磁盘满、权限不足等）。
    IoError(std::io::Error),
    RepositoryLockError,
    NoResult,
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
    /// 当 `book.bid` 已存在时返回 [`AddBookError::BIDAlreadyExists`]。
    fn add(&self, book: Book) -> Result<(), AddBookError>;

    /// 按书名查找，返回所有同名图书；没有匹配时返回 `Ok(vec![])`。
    ///
    /// # 错误
    ///
    /// 仓储访问失败时返回 [`FindBookError`]。
    fn find_book_by_name(&self, name: &str) -> Result<Vec<Book>, FindBookError>;

    /// 按编号 `bid` 精确查找，不存在时返回 `Ok(None)`。
    ///
    /// # 错误
    ///
    /// 仓储访问失败时返回 [`FindBookError`]。
    fn find_book_by_bid(&self, bid: &str) -> Result<Book, FindBookError>;

    /// 更新图书信息，以 `book.bid` 作为定位依据。
    ///
    /// `bid` 是主键，不会被这次更新改动：传入的 `bid` 只用于定位，
    /// 需与原记录保持一致。
    ///
    /// # 错误
    ///
    /// 当 `bid` 不存在时返回 [`UpdateBookError::NonExistedBID`]，
    /// 持久化实现写盘失败时返回 [`UpdateBookError::IoError`]。
    fn update(&self, book: Book) -> Result<(), UpdateBookError>;
}

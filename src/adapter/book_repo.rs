//! 图书仓储接口。

use crate::{
    Book,
    schema::book::{AddBookError, UpdateBookError},
};

/// 图书仓储：定义 [`Book`] 的读写操作。
///
/// 所有实现都应保证 `bid` 在仓储内唯一。
pub trait BookRepository {
    fn new() -> Self;
    /// 新增一本图书。
    ///
    /// # 错误
    ///
    /// 当 `book.bid` 已存在时返回 [`AddBookError::BIDAlreadyExists`]。
    fn add(&mut self, book: Book) -> Result<(), AddBookError>;

    /// new 构造实例
    /// 按书名查找，返回所有同名图书；没有匹配时返回空 `Vec`。
    fn find_book_by_name(&self, name: &str) -> Vec<Book>;

    /// 按编号 `bid` 精确查找，不存在时返回 `None`。
    fn find_book_by_bid(&self, bid: &str) -> Option<Book>;

    /// 更新图书信息，以 `book.bid` 作为定位依据。
    ///
    /// `bid` 是主键，不会被这次更新改动：传入的 `bid` 只用于定位，
    /// 需与原记录保持一致。
    ///
    /// # 错误
    ///
    /// 当 `bid` 不存在时返回 [`UpdateBookError::NonExistedBID`]。
    fn update(&mut self, book: Book) -> Result<(), UpdateBookError>;

    /// 返回仓储内的图书条目数，即不同 `bid` 的图书种数。
    ///
    /// 不是可借总册数——总册数需要对每本图书的 `num` 求和。
    fn get_len(&self) -> usize;
}

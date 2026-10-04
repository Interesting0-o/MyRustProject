//! 管理员仓储接口。

use crate::{
    Librarian,
    schema::librarian::{AddLibrarianError, UpdateLibrarianError},
};

/// 管理员仓储：定义 [`Librarian`] 的读写操作。
///
/// 所有实现都应保证 `account` 在仓储内唯一。
pub trait LibrarianRepository {
    fn new() -> Self;
    /// 新增一名管理员。
    ///
    /// # 错误
    ///
    /// 当 `librarian.account` 已存在时返回 [`AddLibrarianError::AccountAlreadyExists`]。
    fn add(&mut self, librarian: Librarian) -> Result<(), AddLibrarianError>;

    /// 按账号 `account` 精确查找，不存在时返回 `None`。
    fn find_librarian_by_account(&self, account: &str) -> Option<Librarian>;

    /// 按姓名查找，返回所有同名管理员；没有匹配时返回空 `Vec`。
    fn find_librarian_by_name(&self, name: &str) -> Vec<Librarian>;

    /// 更新管理员信息，以 `librarian.account` 作为定位依据。
    ///
    /// `account` 是主键，不会被这次更新改动：传入的 `account` 只用于定位，
    /// 需与原记录保持一致。
    ///
    /// # 错误
    ///
    /// 当 `account` 不存在时返回 [`UpdateLibrarianError::NonExistedAccount`]。
    fn update(&mut self, librarian: Librarian) -> Result<(), UpdateLibrarianError>;

    /// 返回仓储内的管理员条目数；`account` 唯一，因此也是管理员人数。
    fn get_len(&self) -> usize;
}

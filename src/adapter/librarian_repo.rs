//! 管理员仓储接口。

use crate::Librarian;

pub enum AddLibrarianError {
    AccountAlreadyExists,
    RepositoryLockError,
}

pub enum UpdateLibrarianError {
    NonExistedAccount,
    RepositoryLockError,
}

pub enum FindLibrarianError {
    NoResult,
    RepositoryLockError,
}

/// 管理员仓储：定义 [`Librarian`] 的读写操作。
///
/// 所有实现都应保证 `account` 在仓储内唯一。
///
/// 构造不属于本契约：各实现的构造方式不同，由实现自行提供。
pub trait LibrarianRepository {
    /// 新增一名管理员。
    ///
    /// # 错误
    ///
    /// 当 `librarian.account` 已存在时返回 [`AddLibrarianError::AccountAlreadyExists`]。
    fn add(&self, librarian: Librarian) -> Result<(), AddLibrarianError>;

    /// 按账号 `account` 精确查找，不存在时返回 `None`。
    fn find_librarian_by_account(&self, account: &str) -> Result<Librarian, FindLibrarianError>;

    /// 按姓名查找，返回所有同名管理员；没有匹配时返回空 `Vec`。
    fn find_librarian_by_name(&self, name: &str) -> Result<Vec<Librarian>, FindLibrarianError>;

    /// 更新管理员信息，以 `librarian.account` 作为定位依据。
    ///
    /// `account` 是主键，不会被这次更新改动：传入的 `account` 只用于定位，
    /// 需与原记录保持一致。
    ///
    /// # 错误
    ///
    /// 当 `account` 不存在时返回 [`UpdateLibrarianError::NonExistedAccount`]。
    fn update(&self, librarian: Librarian) -> Result<(), UpdateLibrarianError>;
}

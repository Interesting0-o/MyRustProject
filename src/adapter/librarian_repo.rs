//! 管理员仓储接口。

use crate::{Librarian, adapter::BackendError};

/// 新增管理员时可能出现的错误。
#[derive(Debug)]
pub enum AddLibrarianError {
    /// 业务规则：`librarian.account` 已存在。
    AccountAlreadyExists,
    /// 仓储故障（IO、序列化、锁中毒等）。
    Backend(BackendError),
}

/// 更新管理员时可能出现的错误。
#[derive(Debug)]
pub enum UpdateLibrarianError {
    /// 业务规则：`librarian.account` 不存在，更新无对象。
    NonExistedAccount,
    /// 仓储故障（IO、序列化、锁中毒等）。
    Backend(BackendError),
}

/// 查询管理员时可能出现的错误。
///
/// 「没查到」不算仓储故障，用 [`FindLibrarianError::NoResult`] 表达空结果。
#[derive(Debug)]
pub enum FindLibrarianError {
    /// 空结果：没有匹配的管理员。
    NoResult,
    /// 仓储故障（IO、序列化、锁中毒等）。
    Backend(BackendError),
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
    /// 当 `librarian.account` 已存在时返回 [`AddLibrarianError::AccountAlreadyExists`]；
    /// 仓储访问失败时返回 [`AddLibrarianError::Backend`]。
    fn add(&self, librarian: Librarian) -> Result<(), AddLibrarianError>;

    /// 按账号 `account` 精确查找，不存在时返回 [`FindLibrarianError::NoResult`]。
    ///
    /// # 错误
    ///
    /// 仓储访问失败时返回 [`FindLibrarianError::Backend`]。
    fn find_librarian_by_account(&self, account: &str) -> Result<Librarian, FindLibrarianError>;

    /// 按姓名查找，返回所有同名管理员；没有匹配时返回 [`FindLibrarianError::NoResult`]。
    ///
    /// # 错误
    ///
    /// 仓储访问失败时返回 [`FindLibrarianError::Backend`]。
    fn find_librarian_by_name(&self, name: &str) -> Result<Vec<Librarian>, FindLibrarianError>;

    /// 更新管理员信息，以 `librarian.account` 作为定位依据。
    ///
    /// `account` 是主键，不会被这次更新改动：传入的 `account` 只用于定位，
    /// 需与原记录保持一致。
    ///
    /// # 错误
    ///
    /// 当 `account` 不存在时返回 [`UpdateLibrarianError::NonExistedAccount`]；
    /// 仓储访问失败时返回 [`UpdateLibrarianError::Backend`]。
    fn update(&self, librarian: Librarian) -> Result<(), UpdateLibrarianError>;
}

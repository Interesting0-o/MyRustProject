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

/// 管理员仓储的异步契约，供 sqlx 等异步后端实现。
#[async_trait::async_trait]
pub trait AsyncLibrarianRepository: Send {
    /// 新增一名管理员。
    ///
    /// # 错误
    ///
    /// 当 `librarian.account` 已存在时返回 [`AddLibrarianError::AccountAlreadyExists`]。
    async fn add(&mut self, librarian: Librarian) -> Result<(), AddLibrarianError>;

    /// 按账号 `account` 精确查找，不存在时返回 [`FindLibrarianError::NoResult`]。
    async fn find_librarian_by_account(
        &mut self,
        account: &str,
    ) -> Result<Librarian, FindLibrarianError>;

    /// 按姓名查找，返回所有同名管理员；没有匹配时返回 [`FindLibrarianError::NoResult`]。
    async fn find_librarian_by_name(
        &mut self,
        name: &str,
    ) -> Result<Vec<Librarian>, FindLibrarianError>;

    /// 更新管理员信息，以 `librarian.account` 作为定位依据。
    ///
    /// # 错误
    ///
    /// 当 `account` 不存在时返回 [`UpdateLibrarianError::NonExistedAccount`]。
    async fn update(&mut self, librarian: Librarian) -> Result<(), UpdateLibrarianError>;
}

//! 读者仓储接口。

use crate::{Reader, adapter::BackendError};

/// 新增读者时可能出现的错误。
#[derive(Debug)]
pub enum AddReaderError {
    /// 业务规则：`reader.account` 已存在。
    AccountAlreadyExists,
    /// 仓储故障（IO、序列化、锁中毒等）。
    Backend(BackendError),
}

/// 更新读者时可能出现的错误。
#[derive(Debug)]
pub enum UpdateReaderError {
    /// 业务规则：`reader.account` 不存在，更新无对象。
    NonExistedAccount,
    /// 仓储故障（IO、序列化、锁中毒等）。
    Backend(BackendError),
}

/// 查询读者时可能出现的错误。
///
/// 「没查到」不算仓储故障，用 [`FindReaderError::NoResult`] 表达空结果。
#[derive(Debug)]
pub enum FindReaderError {
    /// 空结果：没有匹配的读者。
    NoResult,
    /// 仓储故障（IO、序列化、锁中毒等）。
    Backend(BackendError),
}

/// 读者仓储：定义 [`Reader`] 的读写操作。
///
/// 所有实现都应保证 `account` 在仓储内唯一。
///
/// 构造不属于本契约：各实现的构造方式不同，由实现自行提供。
pub trait ReaderRepository {
    /// 新增一名读者。
    ///
    /// # 错误
    ///
    /// 当 `reader.account` 已存在时返回 [`AddReaderError::AccountAlreadyExists`]；
    /// 仓储访问失败时返回 [`AddReaderError::Backend`]。
    fn add(&self, reader: Reader) -> Result<(), AddReaderError>;

    /// 按账号 `account` 精确查找，不存在时返回 [`FindReaderError::NoResult`]。
    ///
    /// # 错误
    ///
    /// 仓储访问失败时返回 [`FindReaderError::Backend`]。
    fn find_reader_by_account(&self, account: &str) -> Result<Reader, FindReaderError>;

    /// 按姓名查找，返回所有同名读者；没有匹配时返回 [`FindReaderError::NoResult`]。
    ///
    /// # 错误
    ///
    /// 仓储访问失败时返回 [`FindReaderError::Backend`]。
    fn find_reader_by_name(&self, name: &str) -> Result<Vec<Reader>, FindReaderError>;

    /// 更新读者信息，以 `reader.account` 作为定位依据。
    ///
    /// `account` 是主键，不会被这次更新改动：传入的 `account` 只用于定位，
    /// 需与原记录保持一致。
    ///
    /// # 错误
    ///
    /// 当 `account` 不存在时返回 [`UpdateReaderError::NonExistedAccount`]；
    /// 仓储访问失败时返回 [`UpdateReaderError::Backend`]。
    fn update(&self, reader: Reader) -> Result<(), UpdateReaderError>;
}

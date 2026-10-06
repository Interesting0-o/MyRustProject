//! 读者仓储接口。

use crate::{AddReaderError, Reader, UpdateReaderError};

/// 读者仓储：定义 [`Reader`] 的读写操作。
///
/// 所有实现都应保证 `account` 在仓储内唯一。
pub trait ReaderRepository {
    fn new() -> Self;
    /// 新增一名读者。
    ///
    /// # 错误
    ///
    /// 当 `reader.account` 已存在时返回 [`AddReaderError::AccountAlreadyExists`]。
    fn add(&mut self, reader: Reader) -> Result<(), AddReaderError>;

    /// 按账号 `account` 精确查找，不存在时返回 `None`。
    fn find_reader_by_account(&self, account: &str) -> Option<Reader>;

    /// 按姓名查找，返回所有同名读者；没有匹配时返回空 `Vec`。
    fn find_reader_by_name(&self, name: &str) -> Vec<Reader>;

    /// 更新读者信息，以 `reader.account` 作为定位依据。
    ///
    /// `account` 是主键，不会被这次更新改动：传入的 `account` 只用于定位，
    /// 需与原记录保持一致。
    ///
    /// # 错误
    ///
    /// 当 `account` 不存在时返回 [`UpdateReaderError::NonExistedAccount`]。
    fn update(&mut self, reader: Reader) -> Result<(), UpdateReaderError>;

    /// 返回仓储内的读者条目数；`account` 唯一，因此也是读者人数。
    fn get_len(&self) -> usize;
}

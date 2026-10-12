//! 借阅记录仓储接口。

use crate::{BorrowRecord, adapter::BackendError};

/// 新增借阅记录时可能出现的错误。
#[derive(Debug)]
pub enum AddBorrowRecordError {
    /// 业务规则：实现生成的 `br_id` 已存在。
    BRIdIsAlreadyExiste,
    /// 仓储故障（IO、序列化、锁中毒等）。
    Backend(BackendError),
}

/// 删除借阅记录时可能出现的错误。
#[derive(Debug)]
pub enum RemoveBorrowRecordError {
    /// 业务规则：`br_id` 不存在。
    NonExistedBRID,
    /// 仓储故障（IO、序列化、锁中毒等）。
    Backend(BackendError),
}

/// 查询借阅记录时可能出现的错误。
///
/// 「没查到」不算仓储故障，用 [`FindBorrowRecordError::NoResult`] 表达空结果。
#[derive(Debug)]
pub enum FindBorrowRecordError {
    /// 空结果：没有匹配的记录。
    NoResult,
    /// 仓储故障（IO、序列化、锁中毒等）。
    Backend(BackendError),
}

/// 借阅记录仓储：定义 [`BorrowRecord`] 的读写操作。
///
/// 记录一经创建就不再修改，因此没有 `update`：借书时 [`add`](Self::add)，
/// 还书时 [`remove`](Self::remove)。所有实现都应保证 `br_id` 唯一。
///
/// 构造不属于本契约：各实现的构造方式不同，由实现自行提供。
pub trait BorrowRecordRepository {
    /// 新增一条借阅记录。
    ///
    /// `br_id` 由实现自行生成并保证唯一（内存实现使用自增计数器），调用方无需
    /// 提供。不校验「同一读者重复借同一本书」，也不校验 `account` / `bid` 是否存在，
    /// 那属于业务规则，应由 service 层决定。
    ///
    /// # 错误
    ///
    /// 仓储访问失败时返回 [`AddBorrowRecordError::Backend`]。
    fn add(&self, account: &str, bid: &str) -> Result<(), AddBorrowRecordError>;

    /// 按 `br_id` 删除一条借阅记录（还书）。
    ///
    /// # 错误
    ///
    /// 当 `br_id` 不存在时返回 [`RemoveBorrowRecordError::NonExistedBRID`]；
    /// 仓储访问失败时返回 [`RemoveBorrowRecordError::Backend`]。
    fn remove(&self, br_id: &str) -> Result<(), RemoveBorrowRecordError>;

    /// 按记录编号 `br_id` 精确查找，不存在时返回 [`FindBorrowRecordError::NoResult`]。
    ///
    /// # 错误
    ///
    /// 仓储访问失败时返回 [`FindBorrowRecordError::Backend`]。
    fn find_record_by_brid(&self, br_id: &str) -> Result<BorrowRecord, FindBorrowRecordError>;

    /// 查出某本书当前的所有借阅记录；没有时返回 [`FindBorrowRecordError::NoResult`]。
    ///
    /// # 错误
    ///
    /// 仓储访问失败时返回 [`FindBorrowRecordError::Backend`]。
    fn find_records_by_bid(&self, bid: &str) -> Result<Vec<BorrowRecord>, FindBorrowRecordError>;

    /// 查出某位读者当前的所有借阅记录；没有时返回 [`FindBorrowRecordError::NoResult`]。
    ///
    /// # 错误
    ///
    /// 仓储访问失败时返回 [`FindBorrowRecordError::Backend`]。
    fn find_records_by_account(
        &self,
        account: &str,
    ) -> Result<Vec<BorrowRecord>, FindBorrowRecordError>;

    /// 查出某位读者借的某本书的借阅记录；没有时返回 [`FindBorrowRecordError::NoResult`]。
    ///
    /// # 错误
    ///
    /// 仓储访问失败时返回 [`FindBorrowRecordError::Backend`]。
    fn find_records_by_account_and_bid(
        &self,
        account: &str,
        bid: &str,
    ) -> Result<Vec<BorrowRecord>, FindBorrowRecordError>;
}

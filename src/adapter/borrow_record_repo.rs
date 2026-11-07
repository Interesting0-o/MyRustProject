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

/// 借阅记录仓储的异步契约，供 sqlx 等异步后端实现。
#[async_trait::async_trait]
pub trait AsyncBorrowRecordRepository: Send {
    /// 新增一条借阅记录，`br_id` 由实现生成并保证唯一。
    async fn add(&mut self, account: &str, bid: &str) -> Result<(), AddBorrowRecordError>;

    /// 按 `br_id` 删除一条借阅记录（还书）。
    ///
    /// # 错误
    ///
    /// 当 `br_id` 不存在时返回 [`RemoveBorrowRecordError::NonExistedBRID`]。
    async fn remove(&mut self, br_id: &str) -> Result<(), RemoveBorrowRecordError>;

    /// 按记录编号 `br_id` 精确查找，不存在时返回 [`FindBorrowRecordError::NoResult`]。
    async fn find_record_by_brid(
        &mut self,
        br_id: &str,
    ) -> Result<BorrowRecord, FindBorrowRecordError>;

    /// 查出某本书当前的所有借阅记录；没有时返回 [`FindBorrowRecordError::NoResult`]。
    async fn find_records_by_bid(
        &mut self,
        bid: &str,
    ) -> Result<Vec<BorrowRecord>, FindBorrowRecordError>;

    /// 查出某位读者当前的所有借阅记录；没有时返回 [`FindBorrowRecordError::NoResult`]。
    async fn find_records_by_account(
        &mut self,
        account: &str,
    ) -> Result<Vec<BorrowRecord>, FindBorrowRecordError>;

    /// 查出某位读者借的某本书的借阅记录；没有时返回 [`FindBorrowRecordError::NoResult`]。
    async fn find_records_by_account_and_bid(
        &mut self,
        account: &str,
        bid: &str,
    ) -> Result<Vec<BorrowRecord>, FindBorrowRecordError>;
}

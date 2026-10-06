//! 借阅记录仓储接口。

use crate::{BorrowRecord, RemoveBorrowRecordError};

/// 借阅记录仓储：定义 [`BorrowRecord`] 的读写操作。
///
/// 记录一经创建就不再修改，因此没有 `update`：借书时 [`add`](Self::add)，
/// 还书时 [`remove`](Self::remove)。所有实现都应保证 `br_id` 唯一。
pub trait BorrowRecordRepository {
    fn new() -> Self;
    /// 新增一条借阅记录，返回新记录的编号 `br_id`。
    ///
    /// `br_id` 由实现自行生成并保证唯一（内存实现使用自增计数器），调用方无需
    /// 提供，因此本方法不会失败。
    ///
    /// 不校验「同一读者重复借同一本书」，也不校验 `account` / `bid` 是否存在，
    /// 那属于业务规则，应由 service 层决定。
    fn add(&mut self, account: &str, bid: &str) -> String;

    /// 按 `br_id` 删除一条借阅记录（还书）。
    ///
    /// # 错误
    ///
    /// 当 `br_id` 不存在时返回 [`RemoveBorrowRecordError::NonExistedBRID`]。
    fn remove(&mut self, br_id: &str) -> Result<(), RemoveBorrowRecordError>;

    /// 按记录编号 `br_id` 精确查找，不存在时返回 `None`。
    fn find_record_by_brid(&self, br_id: &str) -> Option<BorrowRecord>;

    /// 查出某本书当前的所有借阅记录；没有时返回空 `Vec`。
    fn find_records_by_bid(&self, bid: &str) -> Vec<BorrowRecord>;

    /// 查出某位读者当前的所有借阅记录；没有时返回空 `Vec`。
    fn find_records_by_account(&self, account: &str) -> Vec<BorrowRecord>;

    /// 返回仓储内的借阅记录条数。
    ///
    /// 一条记录代表一册在借图书，因此这也等于当前借出的总册数。
    fn get_len(&self) -> usize;
}

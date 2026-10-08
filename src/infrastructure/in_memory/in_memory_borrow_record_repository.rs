//! [`BorrowRecordRepository`] 的内存实现。

use crate::{BorrowRecord, BorrowRecordRepository, RemoveBorrowRecordError};

/// 基于 `Vec` 的借阅记录仓储，数据仅保存在进程内存中，重启即丢失。
///
/// 所有查找均为线性扫描（O(n)），适用于数据量小的场景。
pub struct InMemoryBorrowRecordRepository {
    records: Vec<BorrowRecord>,
    pk_record: u64,
}

impl InMemoryBorrowRecordRepository {
    /// 构造一个空仓储。构造不属于 [`BorrowRecordRepository`] 契约。
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            pk_record: 0,
        }
    }
}

impl BorrowRecordRepository for InMemoryBorrowRecordRepository {
    fn add(&mut self, account: &str, bid: &str) -> String {
        self.pk_record += 1;
        let br_id = format!("BR_{}", self.pk_record);
        let res = br_id.clone();
        self.records.push(BorrowRecord {
            br_id,
            bid: bid.to_string(),
            account: account.to_string(),
        });
        res
    }

    /// 按 `br_id` 定位后从 `Vec` 中移除，保持其余记录顺序不变。
    fn remove(&mut self, br_id: &str) -> Result<(), RemoveBorrowRecordError> {
        match self.records.iter().position(|r| r.br_id == br_id) {
            Some(index) => {
                self.records.remove(index);
                Ok(())
            }
            None => Err(RemoveBorrowRecordError::NonExistedBRID),
        }
    }

    /// 返回命中记录的克隆副本，调用方修改不会影响仓储内部数据。
    fn find_record_by_brid(&self, br_id: &str) -> Option<BorrowRecord> {
        self.records.iter().find(|r| r.br_id == br_id).cloned()
    }

    /// 收集所有 `bid` 匹配的借阅记录，结果为克隆副本。
    fn find_records_by_bid(&self, bid: &str) -> Vec<BorrowRecord> {
        self.records
            .iter()
            .filter(|r| r.bid == bid)
            .cloned()
            .collect()
    }

    /// 收集所有 `account` 匹配的借阅记录，结果为克隆副本。
    fn find_records_by_account(&self, account: &str) -> Vec<BorrowRecord> {
        self.records
            .iter()
            .filter(|r| r.account == account)
            .cloned()
            .collect()
    }
}

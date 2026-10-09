//! [`BorrowRecordRepository`] 的内存实现。

use std::sync::{Mutex, RwLock};

use crate::{
    BorrowRecord, BorrowRecordRepository, RemoveBorrowRecordError,
    adapter::borrow_record_repo::{AddBorrowRecordError, FindBorrowRecordError},
};

/// 基于 `Vec` 的借阅记录仓储，数据仅保存在进程内存中，重启即丢失。
///
/// 所有查找均为线性扫描（O(n)），适用于数据量小的场景。
pub struct InMemoryBorrowRecordRepository {
    records: RwLock<Vec<BorrowRecord>>,
    pk_record: Mutex<u64>,
}

impl InMemoryBorrowRecordRepository {
    /// 构造一个空仓储。构造不属于 [`BorrowRecordRepository`] 契约。
    pub fn new() -> Self {
        Self {
            records: RwLock::new(Vec::new()),
            pk_record: Mutex::new(0),
        }
    }
}

impl BorrowRecordRepository for InMemoryBorrowRecordRepository {
    fn add(&self, account: &str, bid: &str) -> Result<(), AddBorrowRecordError> {
        let mut records = self
            .records
            .write()
            .map_err(|_| AddBorrowRecordError::RepositoryLockError)?;
        let mut pk_record = self
            .pk_record
            .lock()
            .map_err(|_| AddBorrowRecordError::RepositoryLockError)?;
        *pk_record += 1;
        let br_id = format!("BR{}", *pk_record);
        if records.iter().any(|r| r.br_id == br_id) {
            return Err(AddBorrowRecordError::BRIdIsAlreadyExiste);
        }
        records.push(BorrowRecord {
            br_id,
            bid: bid.to_string(),
            account: account.to_string(),
        });
        Ok(())
    }

    fn remove(&self, br_id: &str) -> Result<(), RemoveBorrowRecordError> {
        let mut records = self
            .records
            .write()
            .map_err(|_| RemoveBorrowRecordError::RepositoryLockError)?;

        match records.iter().position(|r| r.br_id == br_id) {
            Some(index) => {
                records.remove(index);
                Ok(())
            }
            None => Err(RemoveBorrowRecordError::NonExistedBRID),
        }
    }

    fn find_record_by_brid(&self, br_id: &str) -> Result<BorrowRecord, FindBorrowRecordError> {
        let records = self
            .records
            .read()
            .map_err(|_| FindBorrowRecordError::RepositoryLockError)?;
        if let Some(br) = records.iter().find(|b| b.br_id == br_id).cloned() {
            Ok(br)
        } else {
            Err(FindBorrowRecordError::NoResult)
        }
    }

    fn find_records_by_bid(&self, bid: &str) -> Result<Vec<BorrowRecord>, FindBorrowRecordError> {
        let records = self
            .records
            .read()
            .map_err(|_| FindBorrowRecordError::RepositoryLockError)?;
        let query_res: Vec<BorrowRecord> =
            records.iter().filter(|r| r.bid == bid).cloned().collect();

        if query_res.is_empty() {
            Err(FindBorrowRecordError::NoResult)
        } else {
            Ok(query_res)
        }
    }

    fn find_records_by_account(
        &self,
        account: &str,
    ) -> Result<Vec<BorrowRecord>, FindBorrowRecordError> {
        let records = self
            .records
            .read()
            .map_err(|_| FindBorrowRecordError::RepositoryLockError)?;
        let query_res: Vec<BorrowRecord> = records
            .iter()
            .filter(|r| r.account == account)
            .cloned()
            .collect();

        if query_res.is_empty() {
            Err(FindBorrowRecordError::NoResult)
        } else {
            Ok(query_res)
        }
    }

    fn find_records_by_account_and_bid(
        &self,
        account: &str,
        bid: &str,
    ) -> Result<Vec<BorrowRecord>, FindBorrowRecordError> {
        let record = self
            .records
            .read()
            .map_err(|_| FindBorrowRecordError::RepositoryLockError)?;

        let query_res: Vec<BorrowRecord> = record
            .iter()
            .filter(|br| br.account == account && br.bid == bid)
            .cloned()
            .collect();

        if query_res.is_empty() {
            Err(FindBorrowRecordError::NoResult)
        } else {
            Ok(query_res)
        }
    }
}

//! [`BorrowRecordRepository`] 的内存实现。

use crate::{
    BorrowRecord, BorrowRecordRepository, RemoveBorrowRecordError,
    adapter::borrow_record_repo::{AddBorrowRecordError, FindBorrowRecordError},
    infrastructure::{in_memory::InMemoryStore, lock_poisoned},
};

/// 基于内存的借阅记录仓储，数据由 [`InMemoryStore`] 统一持有。
///
/// 所有查找均为线性扫描（O(n)），适用于数据量小的场景。
pub struct InMemoryBorrowRecordRepository<'a> {
    store: &'a InMemoryStore,
}

impl<'a> InMemoryBorrowRecordRepository<'a> {
    /// 构造一个借用底层存储的仓储。构造不属于 [`BorrowRecordRepository`] 契约。
    pub fn new(store: &'a InMemoryStore) -> Self {
        Self { store }
    }
}

impl BorrowRecordRepository for InMemoryBorrowRecordRepository<'_> {
    fn add(&self, account: &str, bid: &str) -> Result<(), AddBorrowRecordError> {
        let mut records = self
            .store
            .borrow_records
            .write()
            .map_err(|_| AddBorrowRecordError::Backend(lock_poisoned()))?;
        let mut next_br_id = self
            .store
            .next_br_id
            .lock()
            .map_err(|_| AddBorrowRecordError::Backend(lock_poisoned()))?;
        *next_br_id += 1;
        let br_id = format!("BR{}", *next_br_id);
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
            .store
            .borrow_records
            .write()
            .map_err(|_| RemoveBorrowRecordError::Backend(lock_poisoned()))?;

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
            .store
            .borrow_records
            .read()
            .map_err(|_| FindBorrowRecordError::Backend(lock_poisoned()))?;
        if let Some(br) = records.iter().find(|b| b.br_id == br_id).cloned() {
            Ok(br)
        } else {
            Err(FindBorrowRecordError::NoResult)
        }
    }

    fn find_records_by_bid(&self, bid: &str) -> Result<Vec<BorrowRecord>, FindBorrowRecordError> {
        let records = self
            .store
            .borrow_records
            .read()
            .map_err(|_| FindBorrowRecordError::Backend(lock_poisoned()))?;
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
            .store
            .borrow_records
            .read()
            .map_err(|_| FindBorrowRecordError::Backend(lock_poisoned()))?;
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
        let records = self
            .store
            .borrow_records
            .read()
            .map_err(|_| FindBorrowRecordError::Backend(lock_poisoned()))?;

        let query_res: Vec<BorrowRecord> = records
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

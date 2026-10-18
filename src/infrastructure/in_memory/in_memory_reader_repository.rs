//! [`ReaderRepository`] 的内存实现。

use crate::{
    AddReaderError, Reader, ReaderRepository, UpdateReaderError,
    adapter::reader_repo::FindReaderError,
    infrastructure::{in_memory::InMemoryStore, lock_poisoned},
};

/// 基于内存的读者仓储，数据由 [`InMemoryStore`] 统一持有。
///
/// 所有查找均为线性扫描（O(n)），适用于数据量小的场景。
pub struct InMemoryReaderRepository<'a> {
    store: &'a InMemoryStore,
}

impl<'a> InMemoryReaderRepository<'a> {
    /// 构造一个借用底层存储的仓储。构造不属于 [`ReaderRepository`] 契约。
    pub fn new(store: &'a InMemoryStore) -> Self {
        Self { store }
    }
}

impl ReaderRepository for InMemoryReaderRepository<'_> {
    /// 先扫描 `account` 是否已被占用，没有才追加到末尾。
    fn add(&self, reader: Reader) -> Result<(), AddReaderError> {
        let mut readers = self
            .store
            .readers
            .write()
            .map_err(|_| AddReaderError::Backend(lock_poisoned()))?;

        if readers.iter().any(|r| r.account == reader.account) {
            Err(AddReaderError::AccountAlreadyExists)
        } else {
            readers.push(reader);
            Ok(())
        }
    }

    /// 按 `account` 定位后整体覆盖（`account` 只能与定位时用的值相同）。
    fn update(&self, reader: Reader) -> Result<(), UpdateReaderError> {
        let mut readers = self
            .store
            .readers
            .write()
            .map_err(|_| UpdateReaderError::Backend(lock_poisoned()))?;
        if let Some(r) = readers.iter_mut().find(|r| r.account == reader.account) {
            *r = reader;
            Ok(())
        } else {
            Err(UpdateReaderError::NonExistedAccount)
        }
    }

    /// 返回命中读者的克隆副本，调用方修改不会影响仓储内部数据。
    fn find_reader_by_account(&self, account: &str) -> Result<Reader, FindReaderError> {
        let readers = self
            .store
            .readers
            .read()
            .map_err(|_| FindReaderError::Backend(lock_poisoned()))?;
        if let Some(reader) = readers.iter().find(|r| r.account == account).cloned() {
            Ok(reader)
        } else {
            Err(FindReaderError::NoResult)
        }
    }

    /// 收集所有姓名完全匹配的读者，结果为克隆副本。
    fn find_reader_by_name(&self, name: &str) -> Result<Vec<Reader>, FindReaderError> {
        let readers = self
            .store
            .readers
            .read()
            .map_err(|_| FindReaderError::Backend(lock_poisoned()))?;
        let query_res: Vec<Reader> = readers.iter().filter(|r| r.name == name).cloned().collect();
        if query_res.is_empty() {
            Err(FindReaderError::NoResult)
        } else {
            Ok(query_res)
        }
    }
}

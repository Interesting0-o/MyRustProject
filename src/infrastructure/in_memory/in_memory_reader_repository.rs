//! [`ReaderRepository`] 的内存实现。
use crate::{
    AddReaderError, Reader, ReaderRepository, UpdateReaderError,
    adapter::reader_repo::FindReaderError,
};
use std::sync::RwLock;

/// 基于 `Vec` 的读者仓储，数据仅保存在进程内存中，重启即丢失。
///
/// 所有查找均为线性扫描（O(n)），适用于数据量小的场景。
pub struct InMemoryReaderRepository {
    readers: RwLock<Vec<Reader>>,
}

impl InMemoryReaderRepository {
    /// 构造一个空仓储。构造不属于 [`ReaderRepository`] 契约。
    pub fn new() -> Self {
        Self {
            readers: RwLock::new(Vec::new()),
        }
    }
}

impl ReaderRepository for InMemoryReaderRepository {
    /// 先扫描 `account` 是否已被占用，没有才追加到末尾。
    fn add(&self, reader: Reader) -> Result<(), AddReaderError> {
        let mut readers = self
            .readers
            .write()
            .map_err(|_| AddReaderError::RepositoryLockError)?;

        if readers.iter().any(|r| r.account == reader.account) {
            return Err(AddReaderError::AccountAlreadyExists);
        } else {
            readers.push(reader);
            Ok(())
        }
    }

    /// 按 `account` 定位后整体覆盖（`account` 只能与定位时用的值相同）。
    fn update(&self, reader: Reader) -> Result<(), UpdateReaderError> {
        let mut readers = self
            .readers
            .write()
            .map_err(|_| UpdateReaderError::RepositoryLockError)?;
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
            .readers
            .read()
            .map_err(|_| FindReaderError::RepositoryLockError)?;
        if let Some(reader) = readers.iter().find(|r| r.account == account).cloned() {
            Ok(reader)
        } else {
            Err(FindReaderError::NoResult)
        }
    }

    /// 收集所有姓名完全匹配的读者，结果为克隆副本。
    fn find_reader_by_name(&self, name: &str) -> Result<Vec<Reader>, FindReaderError> {
        let readers = self
            .readers
            .read()
            .map_err(|_| FindReaderError::RepositoryLockError)?;
        let query_res: Vec<Reader> = readers.iter().filter(|r| r.name == name).cloned().collect();
        if query_res.is_empty() {
            Err(FindReaderError::NoResult)
        } else {
            Ok(query_res)
        }
    }
}

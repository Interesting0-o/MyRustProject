//! [`ReaderRepository`] 的内存实现。

use crate::{
    Reader,
    adapter::reader_repo::ReaderRepository,
    schema::reader::{AddReaderError, UpdateReaderError},
};

/// 基于 `Vec` 的读者仓储，数据仅保存在进程内存中，重启即丢失。
///
/// 所有查找均为线性扫描（O(n)），适用于数据量小的场景。
pub struct InMemoryReaderRepository {
    readers: Vec<Reader>,
    pk_record: usize,
}

impl ReaderRepository for InMemoryReaderRepository {
    fn new() -> Self {
        Self {
            readers: Vec::new(),
            pk_record: 0,
        }
    }
    /// 先扫描 `account` 是否已被占用，没有才追加到末尾。
    fn add(&mut self, reader: Reader) -> Result<(), AddReaderError> {
        match self.readers.iter().find(|r| r.account == reader.account) {
            Some(_) => Err(AddReaderError::AccountAlreadyExists),
            None => {
                self.readers.push(reader);
                Ok(())
            }
        }
    }

    /// 按 `account` 定位后整体覆盖（`account` 只能与定位时用的值相同）。
    fn update(&mut self, reader: Reader) -> Result<(), UpdateReaderError> {
        match self
            .readers
            .iter_mut()
            .find(|r| r.account == reader.account)
        {
            Some(r) => {
                *r = reader;
                Ok(())
            }
            None => Err(UpdateReaderError::NonExistedAccount),
        }
    }

    /// 返回命中读者的克隆副本，调用方修改不会影响仓储内部数据。
    fn find_reader_by_account(&self, account: &str) -> Option<Reader> {
        self.readers.iter().find(|r| r.account == account).cloned()
    }

    /// 收集所有姓名完全匹配的读者，结果为克隆副本。
    fn find_reader_by_name(&self, name: &str) -> Vec<Reader> {
        self.readers
            .iter()
            .filter(|r| r.name == name)
            .cloned()
            .collect()
    }

    fn get_len(&self) -> usize {
        self.readers.len()
    }
}

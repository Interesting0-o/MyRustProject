//! [`LibrarianRepository`] 的内存实现。

use crate::{
    Librarian,
    adapter::librarian_repo::LibrarianRepository,
    schema::librarian::{AddLibrarianError, UpdateLibrarianError},
};

/// 基于 `Vec` 的管理员仓储，数据仅保存在进程内存中，重启即丢失。
///
/// 所有查找均为线性扫描（O(n)），适用于数据量小的场景。
pub struct InMemoryLibrarianRepository {
    librarians: Vec<Librarian>,
    pk_record: usize,
}

impl LibrarianRepository for InMemoryLibrarianRepository {
    fn new() -> Self {
        Self {
            librarians: Vec::new(),
            pk_record: 0,
        }
    }
    /// 先扫描 `account` 是否已被占用，没有才追加到末尾。
    fn add(&mut self, librarian: Librarian) -> Result<(), AddLibrarianError> {
        match self
            .librarians
            .iter()
            .find(|l| l.account == librarian.account)
        {
            Some(_) => Err(AddLibrarianError::AccountAlreadyExists),
            None => {
                self.librarians.push(librarian);
                Ok(())
            }
        }
    }

    /// 按 `account` 定位后整体覆盖（`account` 只能与定位时用的值相同）。
    fn update(&mut self, librarian: Librarian) -> Result<(), UpdateLibrarianError> {
        match self
            .librarians
            .iter_mut()
            .find(|l| l.account == librarian.account)
        {
            Some(l) => {
                *l = librarian;
                Ok(())
            }
            None => Err(UpdateLibrarianError::NonExistedAccount),
        }
    }

    /// 返回命中管理员的克隆副本，调用方修改不会影响仓储内部数据。
    fn find_librarian_by_account(&self, account: &str) -> Option<Librarian> {
        self.librarians
            .iter()
            .find(|l| l.account == account)
            .cloned()
    }

    /// 收集所有姓名完全匹配的管理员，结果为克隆副本。
    fn find_librarian_by_name(&self, name: &str) -> Vec<Librarian> {
        self.librarians
            .iter()
            .filter(|l| l.name == name)
            .cloned()
            .collect()
    }

    fn get_len(&self) -> usize {
        self.librarians.len()
    }
}

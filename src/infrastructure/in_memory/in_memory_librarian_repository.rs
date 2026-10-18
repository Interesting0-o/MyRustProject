//! [`LibrarianRepository`] 的内存实现。

use crate::{
    AddLibrarianError, Librarian, LibrarianRepository, UpdateLibrarianError,
    adapter::librarian_repo::FindLibrarianError,
    infrastructure::{in_memory::InMemoryStore, lock_poisoned},
};

/// 基于内存的管理员仓储，数据由 [`InMemoryStore`] 统一持有。
///
/// 所有查找均为线性扫描（O(n)），适用于数据量小的场景。
pub struct InMemoryLibrarianRepository<'a> {
    store: &'a InMemoryStore,
}

impl<'a> InMemoryLibrarianRepository<'a> {
    /// 构造一个借用底层存储的仓储。构造不属于 [`LibrarianRepository`] 契约。
    pub fn new(store: &'a InMemoryStore) -> Self {
        Self { store }
    }
}

impl LibrarianRepository for InMemoryLibrarianRepository<'_> {
    /// 先扫描 `account` 是否已被占用，没有才追加到末尾。
    fn add(&self, librarian: Librarian) -> Result<(), AddLibrarianError> {
        let mut librarians = self
            .store
            .librarians
            .write()
            .map_err(|_| AddLibrarianError::Backend(lock_poisoned()))?;
        if librarians.iter().any(|l| l.account == librarian.account) {
            Err(AddLibrarianError::AccountAlreadyExists)
        } else {
            librarians.push(librarian);
            Ok(())
        }
    }

    /// 按 `account` 定位后整体覆盖（`account` 只能与定位时用的值相同）。
    fn update(&self, librarian: Librarian) -> Result<(), UpdateLibrarianError> {
        let mut librarians = self
            .store
            .librarians
            .write()
            .map_err(|_| UpdateLibrarianError::Backend(lock_poisoned()))?;
        if let Some(l) = librarians
            .iter_mut()
            .find(|l| l.account == librarian.account)
        {
            *l = librarian;
            Ok(())
        } else {
            Err(UpdateLibrarianError::NonExistedAccount)
        }
    }

    /// 返回命中管理员的克隆副本，调用方修改不会影响仓储内部数据。
    fn find_librarian_by_account(&self, account: &str) -> Result<Librarian, FindLibrarianError> {
        let librarians = self
            .store
            .librarians
            .read()
            .map_err(|_| FindLibrarianError::Backend(lock_poisoned()))?;
        if let Some(l) = librarians.iter().find(|l| l.account == account).cloned() {
            Ok(l)
        } else {
            Err(FindLibrarianError::NoResult)
        }
    }

    /// 收集所有姓名完全匹配的管理员，结果为克隆副本。
    fn find_librarian_by_name(&self, name: &str) -> Result<Vec<Librarian>, FindLibrarianError> {
        let librarians = self
            .store
            .librarians
            .read()
            .map_err(|_| FindLibrarianError::Backend(lock_poisoned()))?;
        let query_res: Vec<Librarian> = librarians
            .iter()
            .filter(|l| l.name == name)
            .cloned()
            .collect();
        if query_res.is_empty() {
            Err(FindLibrarianError::NoResult)
        } else {
            Ok(query_res)
        }
    }
}

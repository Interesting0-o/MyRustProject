//! [`LibrarianRepository`] 的内存实现。

use crate::{
    AddLibrarianError, Librarian, LibrarianRepository, UpdateLibrarianError,
    adapter::librarian_repo::FindLibrarianError,
};
use std::sync::RwLock;
/// 基于 `Vec` 的管理员仓储，数据仅保存在进程内存中，重启即丢失。
///
/// 所有查找均为线性扫描（O(n)），适用于数据量小的场景。
pub struct InMemoryLibrarianRepository {
    librarians: RwLock<Vec<Librarian>>,
}

impl InMemoryLibrarianRepository {
    /// 构造一个空仓储。构造不属于 [`LibrarianRepository`] 契约。
    pub fn new() -> Self {
        Self {
            librarians: RwLock::new(Vec::new()),
        }
    }
}

impl LibrarianRepository for InMemoryLibrarianRepository {
    /// 先扫描 `account` 是否已被占用，没有才追加到末尾。
    fn add(&self, librarian: Librarian) -> Result<(), AddLibrarianError> {
        let mut librarians = self
            .librarians
            .write()
            .map_err(|_| AddLibrarianError::RepositoryLockError)?;
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
            .librarians
            .write()
            .map_err(|_| UpdateLibrarianError::RepositoryLockError)?;
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
            .librarians
            .read()
            .map_err(|_| FindLibrarianError::RepositoryLockError)?;
        if let Some(l) = librarians.iter().find(|l| l.account == account).cloned() {
            Ok(l)
        } else {
            Err(FindLibrarianError::NoResult)
        }
    }

    /// 收集所有姓名完全匹配的管理员，结果为克隆副本。
    fn find_librarian_by_name(&self, name: &str) -> Result<Vec<Librarian>, FindLibrarianError> {
        let librarians = self
            .librarians
            .read()
            .map_err(|_| FindLibrarianError::RepositoryLockError)?;
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

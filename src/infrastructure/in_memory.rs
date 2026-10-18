//! 基于内存的仓储实现：数据只存在进程内，重启即丢失。

pub mod in_memory_book_repository;
pub mod in_memory_borrow_record_repository;
pub mod in_memory_librarian_repository;
pub mod in_memory_reader_repository;

use std::sync::{Mutex, RwLock};

use crate::{
    Book, BookRepository, BorrowRecord, BorrowRecordRepository, Librarian, LibrarianRepository,
    Reader, ReaderRepository, RepositoryGroup, UnitOfWork, UnitOfWorkError,
};

/// 内存仓储共享的底层数据。
///
/// 与 sqlite 的 `Connection` 对应：所有内存仓储都借用同一份数据，
/// 由 [`InMemoryUnitOfWork`] 统一创建并在事务边界内提供。
pub struct InMemoryStore {
    books: RwLock<Vec<Book>>,
    borrow_records: RwLock<Vec<BorrowRecord>>,
    next_br_id: Mutex<u64>,
    readers: RwLock<Vec<Reader>>,
    librarians: RwLock<Vec<Librarian>>,
}

struct InMemorySnapshot {
    books: Vec<Book>,
    borrow_records: Vec<BorrowRecord>,
    next_br_id: u64,
    readers: Vec<Reader>,
    librarians: Vec<Librarian>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            books: RwLock::new(Vec::new()),
            borrow_records: RwLock::new(Vec::new()),
            next_br_id: Mutex::new(0),
            readers: RwLock::new(Vec::new()),
            librarians: RwLock::new(Vec::new()),
        }
    }

    fn snapshot(&self) -> InMemorySnapshot {
        InMemorySnapshot {
            books: self.books.read().unwrap_or_else(|e| e.into_inner()).clone(),
            borrow_records: self
                .borrow_records
                .read()
                .unwrap_or_else(|e| e.into_inner())
                .clone(),
            next_br_id: *self.next_br_id.lock().unwrap_or_else(|e| e.into_inner()),
            readers: self
                .readers
                .read()
                .unwrap_or_else(|e| e.into_inner())
                .clone(),
            librarians: self
                .librarians
                .read()
                .unwrap_or_else(|e| e.into_inner())
                .clone(),
        }
    }

    fn restore(&mut self, snapshot: InMemorySnapshot) {
        *self.books.get_mut().unwrap_or_else(|e| e.into_inner()) = snapshot.books;
        *self
            .borrow_records
            .get_mut()
            .unwrap_or_else(|e| e.into_inner()) = snapshot.borrow_records;
        *self.next_br_id.get_mut().unwrap_or_else(|e| e.into_inner()) = snapshot.next_br_id;
        *self.readers.get_mut().unwrap_or_else(|e| e.into_inner()) = snapshot.readers;
        *self.librarians.get_mut().unwrap_or_else(|e| e.into_inner()) = snapshot.librarians;
    }
}

impl Default for InMemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

/// 内存版工作单元：持有一份 [`InMemoryStore`]。
pub struct InMemoryUnitOfWork {
    store: InMemoryStore,
}

impl InMemoryUnitOfWork {
    pub fn new() -> Self {
        Self {
            store: InMemoryStore::new(),
        }
    }
}

impl Default for InMemoryUnitOfWork {
    fn default() -> Self {
        Self::new()
    }
}

impl UnitOfWork for InMemoryUnitOfWork {
    fn repositories(&self) -> Box<dyn RepositoryGroup + '_> {
        Box::new(InMemoryRepositoryGroup::new(&self.store))
    }

    fn transaction<T, E>(
        &mut self,
        f: impl FnOnce(&dyn RepositoryGroup) -> Result<T, E>,
    ) -> Result<T, UnitOfWorkError<E>> {
        let snapshot = self.store.snapshot();
        let result = {
            let repo_group = InMemoryRepositoryGroup::new(&self.store);
            f(&repo_group)
        };
        match result {
            Ok(value) => Ok(value),
            Err(error) => {
                self.store.restore(snapshot);
                Err(UnitOfWorkError::Transaction(error))
            }
        }
    }
}

pub struct InMemoryRepositoryGroup<'a> {
    store: &'a InMemoryStore,
}

impl<'a> InMemoryRepositoryGroup<'a> {
    pub fn new(store: &'a InMemoryStore) -> Self {
        Self { store }
    }
}

impl RepositoryGroup for InMemoryRepositoryGroup<'_> {
    fn book(&self) -> Box<dyn BookRepository + '_> {
        Box::new(in_memory_book_repository::InMemoryBookRepository::new(
            self.store,
        ))
    }

    fn borrow_record(&self) -> Box<dyn BorrowRecordRepository + '_> {
        Box::new(
            in_memory_borrow_record_repository::InMemoryBorrowRecordRepository::new(self.store),
        )
    }

    fn reader(&self) -> Box<dyn ReaderRepository + '_> {
        Box::new(in_memory_reader_repository::InMemoryReaderRepository::new(
            self.store,
        ))
    }

    fn librarian(&self) -> Box<dyn LibrarianRepository + '_> {
        Box::new(in_memory_librarian_repository::InMemoryLibrarianRepository::new(self.store))
    }
}

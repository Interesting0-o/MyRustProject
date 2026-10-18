use crate::{
    AddBookError, Book, BookRepository, FindBookError, UpdateBookError,
    infrastructure::{in_memory::InMemoryStore, lock_poisoned},
};

/// 基于内存的图书仓储，数据由 [`InMemoryStore`] 统一持有。
pub struct InMemoryBookRepository<'a> {
    store: &'a InMemoryStore,
}

impl<'a> InMemoryBookRepository<'a> {
    pub fn new(store: &'a InMemoryStore) -> Self {
        Self { store }
    }
}

impl BookRepository for InMemoryBookRepository<'_> {
    fn add(&self, book: Book) -> Result<(), AddBookError> {
        let mut books = self
            .store
            .books
            .write()
            .map_err(|_| AddBookError::Backend(lock_poisoned()))?;
        if books.iter().any(|b| b.bid == book.bid) {
            Err(AddBookError::BIDAlreadyExists)
        } else {
            books.push(book);
            Ok(())
        }
    }

    fn update(&self, book: Book) -> Result<(), UpdateBookError> {
        let mut books = self
            .store
            .books
            .write()
            .map_err(|_| UpdateBookError::Backend(lock_poisoned()))?;
        if let Some(b) = books.iter_mut().find(|b| b.bid == book.bid) {
            *b = book;
            Ok(())
        } else {
            Err(UpdateBookError::NonExistedBID)
        }
    }

    fn find_book_by_bid(&self, bid: &str) -> Result<Book, FindBookError> {
        let books = self
            .store
            .books
            .read()
            .map_err(|_| FindBookError::Backend(lock_poisoned()))?;
        if let Some(book) = books.iter().find(|book| book.bid == bid).cloned() {
            Ok(book)
        } else {
            Err(FindBookError::NoResult)
        }
    }

    fn find_book_by_name(&self, name: &str) -> Result<Vec<Book>, FindBookError> {
        let books = self
            .store
            .books
            .read()
            .map_err(|_| FindBookError::Backend(lock_poisoned()))?;
        let query_res: Vec<Book> = books
            .iter()
            .filter(|book| book.name == name)
            .cloned()
            .collect();
        if query_res.is_empty() {
            Err(FindBookError::NoResult)
        } else {
            Ok(query_res)
        }
    }
}

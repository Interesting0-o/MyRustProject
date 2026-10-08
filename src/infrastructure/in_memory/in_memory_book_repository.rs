use crate::{AddBookError, Book, BookRepository, FindBookError, UpdateBookError};
use std::sync::RwLock;

pub struct InMemoryBookRepository {
    books: RwLock<Vec<Book>>,
}

impl InMemoryBookRepository {
    pub fn new() -> Self {
        Self {
            books: RwLock::new(Vec::new()),
        }
    }
}
impl Default for InMemoryBookRepository {
    fn default() -> Self {
        Self::new()
    }
}
impl BookRepository for InMemoryBookRepository {
    fn add(&self, book: Book) -> Result<(), AddBookError> {
        let mut books = self
            .books
            .write()
            .map_err(|_| AddBookError::RepositoryLockError)?;
        if books.iter().any(|b| b.bid == book.bid) {
            Err(AddBookError::BIDAlreadyExists)
        } else {
            books.push(book);
            Ok(())
        }
    }

    fn update(&self, book: Book) -> Result<(), UpdateBookError> {
        let mut books = self
            .books
            .write()
            .map_err(|_| UpdateBookError::RepositoryLockError)?;
        if let Some(b) = books.iter_mut().find(|b| b.bid == book.bid) {
            *b = book;
            Ok(())
        } else {
            Err(UpdateBookError::NonExistedBID)
        }
    }

    fn find_book_by_bid(&self, bid: &str) -> Result<Book, FindBookError> {
        let books = self
            .books
            .read()
            .map_err(|_| FindBookError::RepositoryLockError)?;
        if let Some(book) = books.iter().find(|book| book.bid == bid).cloned() {
            Ok(book)
        } else {
            Err(FindBookError::NoResult)
        }
    }

    fn find_book_by_name(&self, name: &str) -> Result<Vec<Book>, FindBookError> {
        let books = self
            .books
            .read()
            .map_err(|_| FindBookError::RepositoryLockError)?;
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

//! [`BookRepository`] 的内存实现。

use crate::{AddBookError, Book, BookRepository, UpdateBookError};

/// 基于 `Vec` 的图书仓储，数据仅保存在进程内存中，重启即丢失。
///
/// 所有查找均为线性扫描（O(n)），适用于数据量小的场景。
pub struct InMemoryBookRepository {
    books: Vec<Book>,
}

impl BookRepository for InMemoryBookRepository {
    fn new() -> Self {
        Self { books: Vec::new() }
    }

    /// 先扫描是否已存在相同 `bid`，不存在才追加到末尾。
    fn add(&mut self, book: Book) -> Result<(), AddBookError> {
        match self.books.iter().position(|b| b.bid == book.bid) {
            Some(_) => Err(AddBookError::BIDAlreadyExists),
            None => {
                self.books.push(book);
                Ok(())
            }
        }
    }

    /// 按 `bid` 找到对应元素后整体覆盖。
    fn update(&mut self, book: Book) -> Result<(), UpdateBookError> {
        match self.books.iter_mut().find(|b| b.bid == book.bid) {
            Some(b) => {
                *b = book;
                Ok(())
            }
            None => Err(UpdateBookError::NonExistedBID),
        }
    }

    /// 返回命中图书的克隆副本，调用方修改不会影响仓储内部数据。
    fn find_book_by_bid(&self, bid: &str) -> Option<Book> {
        self.books.iter().find(|book| book.bid == bid).cloned()
    }

    /// 收集所有书名完全匹配的图书，结果为克隆副本。
    fn find_book_by_name(&self, name: &str) -> Vec<Book> {
        self.books
            .iter()
            .filter(|book| book.name == name)
            .cloned()
            .collect()
    }

    fn get_len(&self) -> usize {
        self.books.len()
    }
}

use crate::{
    AddBookError, Book, BookRepository,
    utils::{formatter::format_book_for_storage, generator::generator_book_from_str},
};
use std::io::Write;
use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader},
    sync::RwLock,
};

pub struct TxtBaseBookRepository {
    books: RwLock<Vec<Book>>,
    repo_path: String,
}
pub enum TXTBaseNewError {
    PathNotFound,
    PathIsEmpty,
}

impl TxtBaseBookRepository {
    pub fn new(path: &str) -> Result<Self, TXTBaseNewError> {
        if path.trim().is_empty() {
            return Err(TXTBaseNewError::PathIsEmpty);
        }

        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
            .unwrap_or_else(|e| panic!("无法发开该文件{e}"));
        let books = BufReader::new(f)
            .lines()
            .map_while(Result::ok)
            .filter_map(|b| generator_book_from_str(&b).ok())
            .collect();
        Ok(Self {
            books: RwLock::new(books),
            repo_path: path.to_string(),
        })
    }
    fn persist(&self) -> std::io::Result<()> {
        let tmp_path = format!("{}.tmp", self.repo_path);
        let mut file = File::create(&tmp_path)?;
        let books = self.books.write().unwrap();
        for book in books.iter() {
            writeln!(file, "{}", format_book_for_storage(book))?;
        }
        file.sync_all()?;
        fs::rename(&tmp_path, &self.repo_path)
    }
}

impl BookRepository for TxtBaseBookRepository {
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

    fn update(&self, _book: Book) -> Result<(), crate::UpdateBookError> {
        todo!();
    }
    fn find_book_by_bid(&self, _bid: &str) -> Result<Book, crate::FindBookError> {
        todo!();
    }
    fn find_book_by_name(&self, _name: &str) -> Result<Vec<Book>, crate::FindBookError> {
        todo!();
    }
}

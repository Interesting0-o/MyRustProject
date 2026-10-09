use crate::{
    AddBookError, Book, BookRepository, FindBookError, UpdateBookError,
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
            let deal_err = |e| AddBookError::IoError(e);
            books.push(book);
            let tmp_path = format!("{}.tmp", self.repo_path);
            let mut file = File::create(&tmp_path).map_err(deal_err)?;
            for b in books.iter() {
                write!(file, "{}", format_book_for_storage(b)).map_err(deal_err)?;
            }
            file.sync_all().map_err(deal_err)?;
            fs::rename(&tmp_path, &self.repo_path).map_err(deal_err)?;
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
            let deal_err = |e| UpdateBookError::IoError(e);
            let tmp_path = format!("{}.tmp", self.repo_path);
            let mut file = File::create(&tmp_path).map_err(deal_err)?;
            for b in books.iter() {
                write!(file, "{}", format_book_for_storage(b)).map_err(deal_err)?;
            }
            file.sync_all().map_err(deal_err)?;
            fs::rename(&tmp_path, &self.repo_path).map_err(deal_err)?;
            Ok(())
        } else {
            Err(UpdateBookError::NonExistedBID)
        }
    }
    fn find_book_by_bid(&self, bid: &str) -> Result<Book, FindBookError> {
        let book = self
            .books
            .read()
            .map_err(|_| FindBookError::RepositoryLockError)?;
        if let Some(b) = book.iter().find(|b| b.bid == bid).cloned() {
            Ok(b)
        } else {
            Err(FindBookError::NoResult)
        }
    }
    fn find_book_by_name(&self, name: &str) -> Result<Vec<Book>, crate::FindBookError> {
        let books = self
            .books
            .read()
            .map_err(|_| FindBookError::RepositoryLockError)?;
        let query_res: Vec<Book> = books.iter().filter(|b| b.name == name).cloned().collect();

        if query_res.is_empty() {
            Err(FindBookError::NoResult)
        } else {
            Ok(query_res)
        }
    }
}

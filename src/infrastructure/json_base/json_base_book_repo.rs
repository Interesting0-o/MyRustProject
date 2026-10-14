use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Write},
    sync::RwLock,
};

use crate::{
    AddBookError, Book, BookRepository, FindBookError, UpdateBookError,
    infrastructure::{json_base::JSONBaseNewError, lock_poisoned},
};

#[derive(Debug, serde::Serialize)]
pub struct JSONBaseBookRepository {
    books: RwLock<Vec<Book>>,
    repo_path: String,
}

impl JSONBaseBookRepository {
    pub fn new(path: &str) -> Result<Self, JSONBaseNewError> {
        if path.trim().is_empty() {
            return Err(JSONBaseNewError::PathIsEmpty);
        }

        let file = File::open(path).map_err(JSONBaseNewError::OpenFailed)?;
        let books: Vec<Book> = serde_json::from_reader(BufReader::new(file))
            .map_err(|_| JSONBaseNewError::ParseFailed)?;
        Ok(Self {
            books: RwLock::new(books),
            repo_path: path.to_string(),
        })
    }
}

impl BookRepository for JSONBaseBookRepository {
    fn add(&self, book: Book) -> Result<(), AddBookError> {
        let mut books = self
            .books
            .write()
            .map_err(|_| AddBookError::Backend(lock_poisoned()))?;

        if books.iter().any(|b| b.bid == book.bid) {
            return Err(AddBookError::BIDAlreadyExists);
        }
        books.push(book);

        let deal_err = |e| AddBookError::Backend(Box::new(e));
        let tmp_path = format!("{}.tmp", self.repo_path);
        let file = File::create(&tmp_path).map_err(deal_err)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &*books)
            .map_err(|e| AddBookError::Backend(Box::new(e)))?;
        writer.flush().map_err(deal_err)?;
        writer.get_ref().sync_all().map_err(deal_err)?;
        fs::rename(&tmp_path, &self.repo_path).map_err(deal_err)?;
        Ok(())
    }

    fn update(&self, book: Book) -> Result<(), crate::UpdateBookError> {
        let mut books = self
            .books
            .write()
            .map_err(|_| UpdateBookError::Backend(lock_poisoned()))?;
        if let Some(b) = books.iter_mut().find(|tmp_b| tmp_b.bid == book.bid) {
            *b = book;
            let deal_err = |e| UpdateBookError::Backend(Box::new(e));
            let tmp_path = format!("{}.tmp", self.repo_path);
            let file = File::create(&tmp_path).map_err(deal_err)?;
            let mut writer = BufWriter::new(file);
            serde_json::to_writer(&mut writer, &*books)
                .map_err(|e| UpdateBookError::Backend(Box::new(e)))?;
            writer.flush().map_err(deal_err)?;
            writer.get_ref().sync_all().map_err(deal_err)?;
            fs::rename(&tmp_path, &self.repo_path).map_err(deal_err)?;

            Ok(())
        } else {
            Err(UpdateBookError::NonExistedBID)
        }
    }

    fn find_book_by_bid(&self, bid: &str) -> Result<Book, crate::FindBookError> {
        let books = self
            .books
            .read()
            .map_err(|_| FindBookError::Backend(lock_poisoned()))?;
        if let Some(book) = books.iter().find(|b| b.bid == bid).cloned() {
            Ok(book)
        } else {
            Err(FindBookError::NoResult)
        }
    }

    fn find_book_by_name(&self, name: &str) -> Result<Vec<Book>, crate::FindBookError> {
        let books = self
            .books
            .read()
            .map_err(|_| FindBookError::Backend(lock_poisoned()))?;
        let query_res = books
            .iter()
            .filter(|b| b.name == name)
            .cloned()
            .collect::<Vec<Book>>();
        if query_res.is_empty() {
            Err(FindBookError::NoResult)
        } else {
            Ok(query_res)
        }
    }
}

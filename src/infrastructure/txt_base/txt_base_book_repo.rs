use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
};

use crate::{
    AddBookError, Book, BookRepository, UpdateBookError,
    utils::{formatter::format_book_for_storage, generator::generator_book_from_str},
};

/// 图书数据文件，相对进程工作目录。
const BOOKS_PATH: &str = "resource/books.txt";

pub struct TXTBaseBookRepository {
    books: Vec<Book>,
}

impl TXTBaseBookRepository {
    /// 把内存里的全部图书原子地写回文件。
    ///
    /// 先写同目录下的临时文件，`sync_all` 落盘后再 `rename` 覆盖正式文件：
    /// 中途失败或崩溃时正式文件保持旧内容，不会留下半截数据。
    fn persist(&self) -> std::io::Result<()> {
        let tmp_path = format!("{BOOKS_PATH}.tmp");
        let mut file = File::create(&tmp_path)?;
        for book in &self.books {
            writeln!(file, "{}", format_book_for_storage(book))?;
        }
        file.sync_all()?;
        fs::rename(&tmp_path, BOOKS_PATH)
    }
}

impl BookRepository for TXTBaseBookRepository {
    fn new() -> Self {
        // 文件不存在时自动创建空文件；注意 create 必须搭配 write，否则 open 会报错
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(BOOKS_PATH)
            .unwrap_or_else(|e| panic!("无法打开 {BOOKS_PATH}: {e}"));

        let books: Vec<Book> = BufReader::new(file)
            .lines()
            // 遇到读取错误立刻停止读取（避免无限循环）
            .map_while(Result::ok)
            // 解析错误可以安全跳过，不会死循环
            .filter_map(|l| generator_book_from_str(&l).ok())
            .collect();

        Self { books }
    }

    /// 先查重，再把新书推进内存并整体落盘。
    ///
    /// 落盘失败时把刚推进去的记录弹出，保证内存与文件不会各说各话。
    fn add(&mut self, book: Book) -> Result<(), AddBookError> {
        if self.books.iter().any(|b| b.bid == book.bid) {
            return Err(AddBookError::BIDAlreadyExists);
        }

        self.books.push(book);
        if let Err(e) = self.persist() {
            self.books.pop();
            return Err(AddBookError::IoError(e));
        }
        Ok(())
    }

    /// 收集所有书名完全匹配的图书，结果为克隆副本。
    fn find_book_by_name(&self, name: &str) -> Vec<Book> {
        self.books
            .iter()
            .filter(|book| book.name == name)
            .cloned()
            .collect()
    }

    /// 返回命中图书的克隆副本，调用方修改不会影响仓储内部数据。
    fn find_book_by_bid(&self, bid: &str) -> Option<Book> {
        self.books.iter().find(|book| book.bid == bid).cloned()
    }

    /// 按 `bid` 定位并整体覆盖，随后落盘。
    ///
    /// `bid` 是主键，覆盖前后位置不变。落盘失败时把旧记录放回原位，
    /// 保证内存与文件不会各说各话。
    fn update(&mut self, book: Book) -> Result<(), UpdateBookError> {
        let Some(idx) = self.books.iter().position(|b| b.bid == book.bid) else {
            return Err(UpdateBookError::NonExistedBID);
        };

        let old = std::mem::replace(&mut self.books[idx], book);
        if let Err(e) = self.persist() {
            self.books[idx] = old;
            return Err(UpdateBookError::IoError(e));
        }
        Ok(())
    }

    fn get_len(&self) -> usize {
        self.books.len()
    }
}

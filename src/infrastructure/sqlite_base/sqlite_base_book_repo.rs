use rusqlite::{Connection, OptionalExtension, ffi::SQLITE_CONSTRAINT_PRIMARYKEY, params};

use crate::{
    AddBookError, Book, BookRepository, FindBookError, UpdateBookError,
    infrastructure::sqlite_base::{ForSqlRepo, SqliteBaseNewError},
};

pub struct SqliteBaseBookRepository<'a, T>
where
    T: ForSqlRepo,
{
    book_conn: &'a T,
}

impl<'a, T> SqliteBaseBookRepository<'a, T>
where
    T: ForSqlRepo,
{
    pub fn new(conn: &'a T) -> Self {
        Self { book_conn: conn }
    }
}

pub fn new_book_table(path: &str) -> Result<Connection, SqliteBaseNewError> {
    if path.trim().is_empty() {
        return Err(SqliteBaseNewError::PathIsEmpty);
    }
    let conn = Connection::open(path).map_err(SqliteBaseNewError::ConnectError)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS book (
                    bid  TEXT  PRIMARY KEY,
                    name TEXT NOT NULL,
                    author TEXT NOT NULL,
                    price INTEGER DEFAULT 0 CHECK(price>=0),
                    num INTEGER DEFAULT 0 CHECK(num>=0)
            );",
        [],
    )
    .map_err(SqliteBaseNewError::ConnectError)?;
    Ok(conn)
}

impl<'a, T> BookRepository for SqliteBaseBookRepository<'a, T>
where
    T: ForSqlRepo,
{
    fn add(&self, book: Book) -> Result<(), AddBookError> {
        self.book_conn
            .conn()
            .execute(
                "INSERT INTO book (bid,name,author,price,num) VALUES (?1,?2,?3,?4,?5);",
                params![book.bid, book.name, book.author, book.price, book.num],
            )
            .map_err(|e| match e {
                rusqlite::Error::SqliteFailure(err, _)
                    if err.extended_code == SQLITE_CONSTRAINT_PRIMARYKEY =>
                {
                    AddBookError::BIDAlreadyExists
                }

                other => AddBookError::Backend(other.into()),
            })?;
        Ok(())
    }

    fn update(&self, book: Book) -> Result<(), UpdateBookError> {
        let row = self
            .book_conn
            .conn()
            .execute(
                "UPDATE book SET author=?1 , name=?2 , num=?3 ,price=?4 where bid = ?5 ;",
                params![book.author, book.name, book.num, book.price, book.bid],
            )
            .map_err(|e| UpdateBookError::Backend(Box::new(e)))?;
        if row == 0 {
            return Err(UpdateBookError::NonExistedBID);
        }
        Ok(())
    }

    fn find_book_by_bid(&self, bid: &str) -> Result<Book, FindBookError> {
        if let Some(book) = self
            .book_conn
            .conn()
            .query_row("SELECT * FROM book WHERE bid = ?1;", params![bid], |row| {
                Ok(Book {
                    bid: row.get("bid")?,
                    author: row.get("author")?,
                    name: row.get("name")?,
                    price: row.get("price")?,
                    num: row.get("num")?,
                })
            })
            .optional()
            .map_err(|e| FindBookError::Backend(Box::new(e)))?
        {
            Ok(book)
        } else {
            Err(FindBookError::NoResult)
        }
    }

    fn find_book_by_name(&self, name: &str) -> Result<Vec<Book>, FindBookError> {
        let mut stmt = self
            .book_conn
            .conn()
            .prepare("SELECT * FROM book WHERE name = ?1;")
            .map_err(|e| FindBookError::Backend(Box::new(e)))?;
        let rows = stmt
            .query_map([name], |row| {
                Ok(Book {
                    bid: row.get("bid")?,
                    author: row.get("author")?,
                    name: row.get("name")?,
                    price: row.get("price")?,
                    num: row.get("num")?,
                })
            })
            .map_err(|e| FindBookError::Backend(Box::new(e)))?;
        let books: Vec<Book> = rows
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| FindBookError::Backend(Box::new(e)))?;
        if books.is_empty() {
            Err(FindBookError::NoResult)
        } else {
            Ok(books)
        }
    }
}

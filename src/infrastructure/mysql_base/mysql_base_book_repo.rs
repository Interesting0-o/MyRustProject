use sqlx::{
    MySqlPool,
    mysql::{MySqlConnection, MySqlDatabaseError},
};

use crate::{
    AddBookError, AsyncBookRepository, Book, FindBookError, UpdateBookError,
    infrastructure::mysql_base::CreateTableError,
};

pub struct MySqlBaseBookRepository<'a> {
    conn: &'a mut MySqlConnection,
}

impl<'a> MySqlBaseBookRepository<'a> {
    pub fn new(conn: &'a mut MySqlConnection) -> Self {
        Self { conn }
    }
}

pub async fn new_book_table(pool: &MySqlPool) -> Result<(), CreateTableError> {
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS books (
            bid VARCHAR(64) PRIMARY KEY,              -- 对应 String，作为主键（假设是 ISBN 或 UUID）
            name VARCHAR(255) NOT NULL,               -- 对应 String，书名
            author VARCHAR(255) NOT NULL,             -- 对应 String，作者
            price BIGINT NOT NULL,                    -- 对应 i64，价格（注意：这里存的是整数）
            num INT UNSIGNED NOT NULL DEFAULT 0       -- 对应 u32，库存数量，无符号整数
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;"#)
        .execute(pool)
        .await.map_err(|_|CreateTableError::ConnectError)?;
    Ok(())
}

#[async_trait::async_trait]
impl<'a> AsyncBookRepository for MySqlBaseBookRepository<'a> {
    async fn add(&mut self, book: Book) -> Result<(), AddBookError> {
        let res = sqlx::query("INSERT INTO books (bid,name,author,price,num) VALUES (?,?,?,?,?)")
            .bind(&book.bid)
            .bind(&book.name)
            .bind(&book.author)
            .bind(&book.price)
            .bind(&book.num)
            .execute(&mut *self.conn)
            .await
            .map_err(|e| {
                if let sqlx::Error::Database(db_err) = &e {
                    if let Some(mysql_err) = db_err.try_downcast_ref::<MySqlDatabaseError>() {
                        if mysql_err.number() == 1062 {
                            return AddBookError::BIDAlreadyExists;
                        }
                    }
                }
                AddBookError::Backend(Box::new(e))
            })?;

        if res.rows_affected() == 1 {
            Ok(())
        } else {
            Err(AddBookError::Backend(Box::new(std::io::Error::other(
                "插入图书失败：未影响任何行",
            ))))
        }
    }

    async fn update(&mut self, book: Book) -> Result<(), UpdateBookError> {
        let res =
            sqlx::query("UPDATE books SET name = ?, author = ?, price = ?, num = ? WHERE bid = ?")
                .bind(&book.name)
                .bind(&book.author)
                .bind(&book.price)
                .bind(&book.num)
                .bind(&book.bid)
                .execute(&mut *self.conn)
                .await
                .map_err(|e| UpdateBookError::Backend(Box::new(e)))?;

        match res.rows_affected() {
            0 => Err(UpdateBookError::NonExistedBID),
            1 => Ok(()),
            n => Err(UpdateBookError::Backend(Box::new(std::io::Error::other(
                format!("更新图书时影响了 {n} 行，预期为 1 行"),
            )))),
        }
    }
    async fn find_book_by_bid(&mut self, bid: &str) -> Result<Book, FindBookError> {
        if let Some(book) = sqlx::query_as::<_, Book>(
            "SELECT author,bid ,name ,price,num FROM books WHERE bid = ? ",
        )
        .bind(bid)
        .fetch_optional(&mut *self.conn)
        .await
        .map_err(|e| FindBookError::Backend(Box::new(e)))?
        {
            Ok(book)
        } else {
            Err(FindBookError::NoResult)
        }
    }

    async fn find_book_by_name(&mut self, name: &str) -> Result<Vec<Book>, FindBookError> {
        let books = sqlx::query_as::<_, Book>(
            "SELECT author, bid, name, price, num FROM books WHERE name = ?",
        )
        .bind(name)
        .fetch_all(&mut *self.conn)
        .await
        .map_err(|e| FindBookError::Backend(Box::new(e)))?;

        if books.is_empty() {
            Err(FindBookError::NoResult)
        } else {
            Ok(books)
        }
    }
}

use sqlx::{
    MySqlPool,
    mysql::{MySqlConnection, MySqlDatabaseError},
};

use crate::{AddReaderError, AsyncReaderRepository, FindReaderError, Reader, UpdateReaderError};

pub struct MySqlBaseReaderRepository<'a> {
    conn: &'a mut MySqlConnection,
}

impl<'a> MySqlBaseReaderRepository<'a> {
    pub fn new(conn: &'a mut MySqlConnection) -> Self {
        Self { conn }
    }
}

#[derive(Debug)]
pub enum CreateTableError {
    ConnectError,
}

pub async fn new_reader_table(pool: &MySqlPool) -> Result<(), CreateTableError> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS readers (
            account VARCHAR(64) PRIMARY KEY,          -- 对应 String，作为主键
            name VARCHAR(255) NOT NULL,               -- 对应 String，姓名
            hash_pwd BIGINT UNSIGNED NOT NULL         -- 对应 u64，密码摘要
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;"#,
    )
    .execute(pool)
    .await
    .map_err(|_| CreateTableError::ConnectError)?;
    Ok(())
}

#[async_trait::async_trait]
impl<'a> AsyncReaderRepository for MySqlBaseReaderRepository<'a> {
    async fn add(&mut self, reader: Reader) -> Result<(), AddReaderError> {
        let res = sqlx::query("INSERT INTO readers (account, name, hash_pwd) VALUES (?, ?, ?)")
            .bind(&reader.account)
            .bind(&reader.name)
            .bind(reader.hash_pwd)
            .execute(&mut *self.conn)
            .await
            .map_err(|e| {
                if let sqlx::Error::Database(db_err) = &e {
                    if let Some(mysql_err) = db_err.try_downcast_ref::<MySqlDatabaseError>() {
                        if mysql_err.number() == 1062 {
                            return AddReaderError::AccountAlreadyExists;
                        }
                    }
                }
                AddReaderError::Backend(Box::new(e))
            })?;

        if res.rows_affected() == 1 {
            Ok(())
        } else {
            Err(AddReaderError::Backend(Box::new(std::io::Error::other(
                "插入读者失败：未影响任何行",
            ))))
        }
    }

    async fn find_reader_by_account(&mut self, account: &str) -> Result<Reader, FindReaderError> {
        sqlx::query_as::<_, Reader>("SELECT account, name, hash_pwd FROM readers WHERE account = ?")
            .bind(account)
            .fetch_optional(&mut *self.conn)
            .await
            .map_err(|e| FindReaderError::Backend(Box::new(e)))?
            .ok_or(FindReaderError::NoResult)
    }

    async fn find_reader_by_name(&mut self, name: &str) -> Result<Vec<Reader>, FindReaderError> {
        let readers = sqlx::query_as::<_, Reader>(
            "SELECT account, name, hash_pwd FROM readers WHERE name = ?",
        )
        .bind(name)
        .fetch_all(&mut *self.conn)
        .await
        .map_err(|e| FindReaderError::Backend(Box::new(e)))?;

        if readers.is_empty() {
            Err(FindReaderError::NoResult)
        } else {
            Ok(readers)
        }
    }

    async fn update(&mut self, reader: Reader) -> Result<(), UpdateReaderError> {
        let res = sqlx::query("UPDATE readers SET name = ?, hash_pwd = ? WHERE account = ?")
            .bind(&reader.name)
            .bind(reader.hash_pwd)
            .bind(&reader.account)
            .execute(&mut *self.conn)
            .await
            .map_err(|e| UpdateReaderError::Backend(Box::new(e)))?;

        match res.rows_affected() {
            0 => Err(UpdateReaderError::NonExistedAccount),
            1 => Ok(()),
            n => Err(UpdateReaderError::Backend(Box::new(std::io::Error::other(
                format!("更新读者时影响了 {n} 行，预期为 1 行"),
            )))),
        }
    }
}

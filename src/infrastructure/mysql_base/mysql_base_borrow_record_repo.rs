use sqlx::{
    MySqlPool,
    mysql::{MySqlConnection, MySqlDatabaseError},
};

use crate::{
    AsyncBorrowRecordRepository, BorrowRecord, RemoveBorrowRecordError,
    adapter::borrow_record_repo::{AddBorrowRecordError, FindBorrowRecordError},
    infrastructure::mysql_base::CreateTableError,
};

pub struct MySqlBaseBorrowRecorRepository<'a> {
    conn: &'a mut MySqlConnection,
}

impl<'a> MySqlBaseBorrowRecorRepository<'a> {
    pub fn new(conn: &'a mut MySqlConnection) -> Self {
        Self { conn }
    }
}

pub async fn new_borrow_record_table(pool: &MySqlPool) -> Result<(), CreateTableError> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS borrow_record (
            br_id VARCHAR(64) PRIMARY KEY,      -- 对应 String，记录编号，全局唯一
            bid VARCHAR(64) NOT NULL,           -- 对应 String，books.bid
            account VARCHAR(64) NOT NULL,       -- 对应 String，readers.account
            INDEX idx_borrow_record_bid (bid),
            INDEX idx_borrow_record_account (account)
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;"#,
    )
    .execute(pool)
    .await
    .map_err(|_| CreateTableError::ConnectError)?;
    Ok(())
}

#[async_trait::async_trait]
impl<'a> AsyncBorrowRecordRepository for MySqlBaseBorrowRecorRepository<'a> {
    async fn add(&mut self, account: &str, bid: &str) -> Result<(), AddBorrowRecordError> {
        let max_id = sqlx::query_scalar::<_, u64>(
            "SELECT CAST(COALESCE(MAX(CAST(SUBSTRING(br_id, 3) AS UNSIGNED)), 0) AS UNSIGNED) \
             FROM borrow_record",
        )
        .fetch_one(&mut *self.conn)
        .await
        .map_err(|e| AddBorrowRecordError::Backend(Box::new(e)))?;

        let br_id = format!("BR{}", max_id + 1);
        let res = sqlx::query("INSERT INTO borrow_record (br_id, bid, account) VALUES (?, ?, ?)")
            .bind(&br_id)
            .bind(bid)
            .bind(account)
            .execute(&mut *self.conn)
            .await
            .map_err(|e| {
                if let sqlx::Error::Database(db_err) = &e
                    && let Some(mysql_err) = db_err.try_downcast_ref::<MySqlDatabaseError>()
                    && mysql_err.number() == 1062
                {
                    return AddBorrowRecordError::BRIdIsAlreadyExiste;
                }
                AddBorrowRecordError::Backend(Box::new(e))
            })?;

        if res.rows_affected() == 1 {
            Ok(())
        } else {
            Err(AddBorrowRecordError::Backend(Box::new(
                std::io::Error::other("插入借阅记录失败：未影响任何行"),
            )))
        }
    }

    async fn remove(&mut self, br_id: &str) -> Result<(), RemoveBorrowRecordError> {
        let res = sqlx::query("DELETE FROM borrow_record WHERE br_id = ?")
            .bind(br_id)
            .execute(&mut *self.conn)
            .await
            .map_err(|e| RemoveBorrowRecordError::Backend(Box::new(e)))?;

        match res.rows_affected() {
            0 => Err(RemoveBorrowRecordError::NonExistedBRID),
            1 => Ok(()),
            n => Err(RemoveBorrowRecordError::Backend(Box::new(
                std::io::Error::other(format!("删除借阅记录时影响了 {n} 行，预期为 1 行")),
            ))),
        }
    }

    async fn find_records_by_account(
        &mut self,
        account: &str,
    ) -> Result<Vec<BorrowRecord>, FindBorrowRecordError> {
        let brs = sqlx::query_as::<_, BorrowRecord>(
            "SELECT br_id,account,bid FROM borrow_record WHERE account = ?",
        )
        .bind(account)
        .fetch_all(&mut *self.conn)
        .await
        .map_err(|e| FindBorrowRecordError::Backend(Box::new(e)))?;

        if brs.is_empty() {
            Err(FindBorrowRecordError::NoResult)
        } else {
            Ok(brs)
        }
    }
    async fn find_record_by_brid(
        &mut self,
        br_id: &str,
    ) -> Result<BorrowRecord, FindBorrowRecordError> {
        if let Some(br) = sqlx::query_as::<_, BorrowRecord>(
            "SELECT br_id,account,bid FROM borrow_record WHERE br_id = ?",
        )
        .bind(br_id)
        .fetch_optional(&mut *self.conn)
        .await
        .map_err(|e| FindBorrowRecordError::Backend(Box::new(e)))?
        {
            Ok(br)
        } else {
            Err(FindBorrowRecordError::NoResult)
        }
    }

    async fn find_records_by_bid(
        &mut self,
        bid: &str,
    ) -> Result<Vec<BorrowRecord>, FindBorrowRecordError> {
        let brs = sqlx::query_as::<_, BorrowRecord>(
            "SELECT br_id,account,bid FROM borrow_record WHERE bid = ?",
        )
        .bind(bid)
        .fetch_all(&mut *self.conn)
        .await
        .map_err(|e| FindBorrowRecordError::Backend(Box::new(e)))?;

        if brs.is_empty() {
            Err(FindBorrowRecordError::NoResult)
        } else {
            Ok(brs)
        }
    }
    async fn find_records_by_account_and_bid(
        &mut self,
        account: &str,
        bid: &str,
    ) -> Result<Vec<BorrowRecord>, FindBorrowRecordError> {
        let brs = sqlx::query_as::<_, BorrowRecord>(
            "SELECT br_id,account,bid FROM borrow_record WHERE account = ? AND bid = ?",
        )
        .bind(account)
        .bind(bid)
        .fetch_all(&mut *self.conn)
        .await
        .map_err(|e| FindBorrowRecordError::Backend(Box::new(e)))?;

        if brs.is_empty() {
            Err(FindBorrowRecordError::NoResult)
        } else {
            Ok(brs)
        }
    }
}

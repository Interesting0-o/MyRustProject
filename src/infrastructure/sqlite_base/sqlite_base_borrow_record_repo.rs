use rusqlite::{Connection, OptionalExtension, Row, ffi::SQLITE_CONSTRAINT_PRIMARYKEY, params};

use crate::{
    BorrowRecord, BorrowRecordRepository, RemoveBorrowRecordError,
    adapter::borrow_record_repo::{AddBorrowRecordError, FindBorrowRecordError},
    infrastructure::sqlite_base::SqliteBaseNewError,
};

pub struct SqliteBaseBorrowRecordRepository<'a> {
    br_conn: &'a Connection,
}

pub fn new_borrow_record_table(path: &str) -> Result<Connection, SqliteBaseNewError> {
    if path.trim().is_empty() {
        return Err(SqliteBaseNewError::PathIsEmpty);
    }
    let conn = Connection::open(path).map_err(SqliteBaseNewError::ConnectError)?;
    create_borrow_record_table(&conn)?;

    Ok(conn)
}

pub(crate) fn create_borrow_record_table(conn: &Connection) -> Result<(), SqliteBaseNewError> {
    conn.pragma_update(None, "foreign_keys", true)
        .map_err(SqliteBaseNewError::ConnectError)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS borrow_record (
                br_id INTEGER PRIMARY KEY AUTOINCREMENT,
                account TEXT NOT NULL,
                bid TEXT NOT NULL,
                FOREIGN KEY (account) REFERENCES reader(account),
                FOREIGN KEY (bid) REFERENCES book(bid)
            );",
        [],
    )
    .map_err(SqliteBaseNewError::ConnectError)?;

    Ok(())
}

impl<'a> SqliteBaseBorrowRecordRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { br_conn: conn }
    }

    fn query_records(
        &self,
        sql: &str,
        params: impl rusqlite::Params,
    ) -> Result<Vec<BorrowRecord>, FindBorrowRecordError> {
        let mut stmt = self
            .br_conn
            .prepare(sql)
            .map_err(|e| FindBorrowRecordError::Backend(Box::new(e)))?;
        let records = stmt
            .query_map(params, read_record)
            .map_err(|e| FindBorrowRecordError::Backend(Box::new(e)))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| FindBorrowRecordError::Backend(Box::new(e)))?;
        if records.is_empty() {
            Err(FindBorrowRecordError::NoResult)
        } else {
            Ok(records)
        }
    }
}

fn read_record(row: &Row<'_>) -> rusqlite::Result<BorrowRecord> {
    Ok(BorrowRecord {
        br_id: format!("BR{}", row.get::<_, i64>("br_id")?),
        bid: row.get("bid")?,
        account: row.get("account")?,
    })
}

fn parse_br_id(br_id: &str) -> Option<i64> {
    br_id.strip_prefix("BR")?.parse().ok()
}

impl BorrowRecordRepository for SqliteBaseBorrowRecordRepository<'_> {
    fn add(&self, account: &str, bid: &str) -> Result<(), AddBorrowRecordError> {
        self.br_conn
            .execute(
                "INSERT INTO borrow_record (account, bid) VALUES (?1, ?2);",
                params![account, bid],
            )
            .map_err(|e| match e {
                rusqlite::Error::SqliteFailure(err, _)
                    if err.extended_code == SQLITE_CONSTRAINT_PRIMARYKEY =>
                {
                    AddBorrowRecordError::BRIdIsAlreadyExiste
                }
                other => AddBorrowRecordError::Backend(Box::new(other)),
            })?;
        Ok(())
    }

    fn remove(&self, br_id: &str) -> Result<(), RemoveBorrowRecordError> {
        let Some(id) = parse_br_id(br_id) else {
            return Err(RemoveBorrowRecordError::NonExistedBRID);
        };
        let row = self
            .br_conn
            .execute("DELETE FROM borrow_record WHERE br_id = ?1;", params![id])
            .map_err(|e| RemoveBorrowRecordError::Backend(Box::new(e)))?;
        if row == 0 {
            Err(RemoveBorrowRecordError::NonExistedBRID)
        } else {
            Ok(())
        }
    }

    fn find_record_by_brid(&self, br_id: &str) -> Result<BorrowRecord, FindBorrowRecordError> {
        let Some(id) = parse_br_id(br_id) else {
            return Err(FindBorrowRecordError::NoResult);
        };
        if let Some(record) = self
            .br_conn
            .query_row(
                "SELECT * FROM borrow_record WHERE br_id = ?1;",
                params![id],
                read_record,
            )
            .optional()
            .map_err(|e| FindBorrowRecordError::Backend(Box::new(e)))?
        {
            Ok(record)
        } else {
            Err(FindBorrowRecordError::NoResult)
        }
    }

    fn find_records_by_bid(&self, bid: &str) -> Result<Vec<BorrowRecord>, FindBorrowRecordError> {
        self.query_records("SELECT * FROM borrow_record WHERE bid = ?1;", params![bid])
    }

    fn find_records_by_account(
        &self,
        account: &str,
    ) -> Result<Vec<BorrowRecord>, FindBorrowRecordError> {
        self.query_records(
            "SELECT * FROM borrow_record WHERE account = ?1;",
            params![account],
        )
    }

    fn find_records_by_account_and_bid(
        &self,
        account: &str,
        bid: &str,
    ) -> Result<Vec<BorrowRecord>, FindBorrowRecordError> {
        self.query_records(
            "SELECT * FROM borrow_record WHERE account = ?1 AND bid = ?2;",
            params![account, bid],
        )
    }
}

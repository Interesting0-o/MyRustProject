use rusqlite::{Connection, OptionalExtension, ffi::SQLITE_CONSTRAINT_PRIMARYKEY, params};

use crate::{
    AddReaderError, FindReaderError, Reader, ReaderRepository, UpdateReaderError,
    infrastructure::sqlite_base::SqliteBaseNewError,
};

#[derive(Debug)]
pub struct SqliteBaseReaderRepository {
    reader_conn: Connection,
}

impl SqliteBaseReaderRepository {
    pub fn new(path: &str) -> Result<Self, SqliteBaseNewError> {
        if path.trim().is_empty() {
            return Err(SqliteBaseNewError::PathIsEmpty);
        }

        let conn = Connection::open(path).map_err(SqliteBaseNewError::ConnectError)?;

        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS reader (
                account TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                hash_pwd INTEGER NOT NULL
            );",
            [],
        )
        .map_err(SqliteBaseNewError::ConnectError)?;
        Ok(Self { reader_conn: conn })
    }
}

impl ReaderRepository for SqliteBaseReaderRepository {
    fn add(&self, reader: Reader) -> Result<(), AddReaderError> {
        self.reader_conn
            .execute(
                "INSERT INTO reader(account,name,hash_pwd) VALUES (?1, ?2, ?3);",
                params![reader.account, reader.name, reader.hash_pwd as i64],
            )
            .map_err(|e| match e {
                rusqlite::Error::SqliteFailure(err, _)
                    if err.extended_code == SQLITE_CONSTRAINT_PRIMARYKEY =>
                {
                    AddReaderError::AccountAlreadyExists
                }
                other => AddReaderError::Backend(Box::new(other)),
            })?;
        Ok(())
    }

    fn update(&self, reader: Reader) -> Result<(), UpdateReaderError> {
        let row = self
            .reader_conn
            .execute(
                "UPDATE reader SET name=?1, hash_pwd = ?2 WHERE account == ?3 ;",
                params![reader.name, reader.hash_pwd as i64, reader.account],
            )
            .map_err(|e| UpdateReaderError::Backend(Box::new(e)))?;
        if row == 0 {
            Err(UpdateReaderError::NonExistedAccount)
        } else {
            Ok(())
        }
    }

    fn find_reader_by_name(&self, name: &str) -> Result<Vec<Reader>, FindReaderError> {
        let mut stmt = self
            .reader_conn
            .prepare("SELECT * FROM reader WHERE name=?1")
            .map_err(|e| FindReaderError::Backend(Box::new(e)))?;

        let query_res = stmt
            .query_map(params![name], |row| {
                Ok(Reader {
                    account: row.get("account")?,
                    hash_pwd: row.get::<_, i64>("hash_pwd")? as u64,
                    name: row.get("name")?,
                })
            })
            .map_err(|e| FindReaderError::Backend(Box::new(e)))?
            .collect::<Result<Vec<Reader>, _>>()
            .map_err(|e| FindReaderError::Backend(Box::new(e)))?;
        if query_res.is_empty() {
            Err(FindReaderError::NoResult)
        } else {
            Ok(query_res)
        }
    }

    fn find_reader_by_account(&self, account: &str) -> Result<Reader, FindReaderError> {
        if let Some(reader) = self
            .reader_conn
            .query_row(
                "SELECT * FROM reader WHERE account = ?1;",
                params![account],
                |row| {
                    Ok(Reader {
                        account: row.get("account")?,
                        hash_pwd: row.get::<_, i64>("hash_pwd")? as u64,
                        name: row.get("name")?,
                    })
                },
            )
            .optional()
            .map_err(|e| FindReaderError::Backend(Box::new(e)))?
        {
            Ok(reader)
        } else {
            Err(FindReaderError::NoResult)
        }
    }
}

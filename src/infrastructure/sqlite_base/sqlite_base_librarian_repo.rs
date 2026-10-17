use rusqlite::{Connection, OptionalExtension, Row, ffi::SQLITE_CONSTRAINT_PRIMARYKEY, params};

use crate::{
    AddLibrarianError, FindLibrarianError, Librarian, LibrarianRepository, UpdateLibrarianError,
    infrastructure::sqlite_base::{ForSqlRepo, SqliteBaseNewError},
};

pub struct SqliteBaseLibrarianRepository<'a, T>
where
    T: ForSqlRepo,
{
    librarian_conn: &'a T,
}

pub fn new_librarian_table(path: &str) -> Result<Connection, SqliteBaseNewError> {
    if path.trim().is_empty() {
        return Err(SqliteBaseNewError::PathIsEmpty);
    }
    let conn = Connection::open(path).map_err(SqliteBaseNewError::ConnectError)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS librarian (
            account TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            hash_pwd INTEGER NOT NULL
        );",
        [],
    )
    .map_err(SqliteBaseNewError::ConnectError)?;
    Ok(conn)
}

impl<'a, T> SqliteBaseLibrarianRepository<'a, T>
where
    T: ForSqlRepo,
{
    pub fn new(conn: &'a T) -> Self {
        Self {
            librarian_conn: conn,
        }
    }
}

fn read_librarian(row: &Row<'_>) -> rusqlite::Result<Librarian> {
    Ok(Librarian {
        account: row.get("account")?,
        name: row.get("name")?,
        hash_pwd: row.get::<_, i64>("hash_pwd")? as u64,
    })
}

impl<'a, T> LibrarianRepository for SqliteBaseLibrarianRepository<'a, T>
where
    T: ForSqlRepo,
{
    fn add(&self, librarian: Librarian) -> Result<(), AddLibrarianError> {
        self.librarian_conn
            .conn()
            .execute(
                "INSERT INTO librarian (account, name, hash_pwd) VALUES (?1, ?2, ?3);",
                params![librarian.account, librarian.name, librarian.hash_pwd as i64],
            )
            .map_err(|e| match e {
                rusqlite::Error::SqliteFailure(err, _)
                    if err.extended_code == SQLITE_CONSTRAINT_PRIMARYKEY =>
                {
                    AddLibrarianError::AccountAlreadyExists
                }
                other => AddLibrarianError::Backend(Box::new(other)),
            })?;
        Ok(())
    }

    fn find_librarian_by_account(&self, account: &str) -> Result<Librarian, FindLibrarianError> {
        if let Some(librarian) = self
            .librarian_conn
            .conn()
            .query_row(
                "SELECT * FROM librarian WHERE account = ?1;",
                params![account],
                read_librarian,
            )
            .optional()
            .map_err(|e| FindLibrarianError::Backend(Box::new(e)))?
        {
            Ok(librarian)
        } else {
            Err(FindLibrarianError::NoResult)
        }
    }

    fn find_librarian_by_name(&self, name: &str) -> Result<Vec<Librarian>, FindLibrarianError> {
        let mut stmt = self
            .librarian_conn
            .conn()
            .prepare("SELECT * FROM librarian WHERE name = ?1;")
            .map_err(|e| FindLibrarianError::Backend(Box::new(e)))?;
        let rows = stmt
            .query_map(params![name], read_librarian)
            .map_err(|e| FindLibrarianError::Backend(Box::new(e)))?;
        let librarians = rows
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| FindLibrarianError::Backend(Box::new(e)))?;
        if librarians.is_empty() {
            Err(FindLibrarianError::NoResult)
        } else {
            Ok(librarians)
        }
    }

    fn update(&self, librarian: Librarian) -> Result<(), UpdateLibrarianError> {
        let row = self
            .librarian_conn
            .conn()
            .execute(
                "UPDATE librarian SET name=?1, hash_pwd=?2 WHERE account = ?3;",
                params![librarian.name, librarian.hash_pwd as i64, librarian.account],
            )
            .map_err(|e| UpdateLibrarianError::Backend(Box::new(e)))?;
        if row == 0 {
            Err(UpdateLibrarianError::NonExistedAccount)
        } else {
            Ok(())
        }
    }
}

use sqlx::mysql::{MySqlConnection, MySqlDatabaseError};

use crate::{
    AddLibrarianError, AsyncLibrarianRepository, FindLibrarianError, Librarian,
    UpdateLibrarianError,
};

pub struct MySqlBaseLibrarianRepository<'a> {
    conn: &'a mut MySqlConnection,
}

impl<'a> MySqlBaseLibrarianRepository<'a> {
    pub fn new(conn: &'a mut MySqlConnection) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl<'a> AsyncLibrarianRepository for MySqlBaseLibrarianRepository<'a> {
    async fn add(&mut self, librarian: Librarian) -> Result<(), AddLibrarianError> {
        let res = sqlx::query("INSERT INTO librarian (account,name,hash_pwd) VALUES (?,?,?) ")
            .bind(&librarian.account)
            .bind(&librarian.name)
            .bind(librarian.hash_pwd)
            .execute(&mut *self.conn)
            .await
            .map_err(|e| {
                if let sqlx::Error::Database(db_err) = &e
                    && let Some(mysql_err) = db_err.try_downcast_ref::<MySqlDatabaseError>()
                    && mysql_err.number() == 1062
                {
                    return AddLibrarianError::AccountAlreadyExists;
                }
                AddLibrarianError::Backend(Box::new(e))
            })?;
        if res.rows_affected() == 1 {
            return Ok(());
        } else {
            return Err(AddLibrarianError::Backend(Box::new(std::io::Error::other(
                "用户数据添加失败",
            ))));
        }
    }
    async fn update(&mut self, librarian: Librarian) -> Result<(), UpdateLibrarianError> {
        let res = sqlx::query("UPDATE librarian SET name=?,hash_pwd=? WHERE account = ?")
            .bind(&librarian.name)
            .bind(librarian.hash_pwd)
            .bind(&librarian.account)
            .execute(&mut *self.conn)
            .await
            .map_err(|e| UpdateLibrarianError::Backend(Box::new(e)))?;

        if res.rows_affected() == 1 {
            return Ok(());
        }

        let exists =
            sqlx::query_scalar::<_, String>("SELECT account FROM librarian WHERE account = ?")
                .bind(&librarian.account)
                .fetch_optional(&mut *self.conn)
                .await
                .map_err(|e| UpdateLibrarianError::Backend(Box::new(e)))?
                .is_some();

        if exists {
            Ok(())
        } else {
            Err(UpdateLibrarianError::NonExistedAccount)
        }
    }
    async fn find_librarian_by_account(
        &mut self,
        account: &str,
    ) -> Result<Librarian, FindLibrarianError> {
        if let Some(librarian) = sqlx::query_as::<_, Librarian>(
            "SELECT account,name,hash_pwd FROM librarian WHERE account =?",
        )
        .bind(account)
        .fetch_optional(&mut *self.conn)
        .await
        .map_err(|e| FindLibrarianError::Backend(Box::new(e)))?
        {
            Ok(librarian)
        } else {
            Err(FindLibrarianError::NoResult)
        }
    }
    async fn find_librarian_by_name(
        &mut self,
        name: &str,
    ) -> Result<Vec<Librarian>, FindLibrarianError> {
        let librarians = sqlx::query_as::<_, Librarian>(
            "SELECT account,name,hash_pwd FROM librarian WHERE name = ?",
        )
        .bind(name)
        .fetch_all(&mut *self.conn)
        .await
        .map_err(|e| FindLibrarianError::Backend(Box::new(e)))?;

        if librarians.is_empty() {
            Err(FindLibrarianError::NoResult)
        } else {
            Ok(librarians)
        }
    }
}

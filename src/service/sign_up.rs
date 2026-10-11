use crate::{
    AddLibrarianError, AddReaderError, Librarian, LibrarianRepository, Reader, ReaderRepository,
    service::login::str_hash,
};

pub enum SignUpRes {
    AccountAlreadyExists,
    /// 写入仓储失败（锁中毒、写持久化数据出错等）。
    RepositoryError,
}

pub fn reader_sign_up<R>(
    reader_repo: &R,
    name: &str,
    account: &str,
    pwd: &str,
) -> Result<(), SignUpRes>
where
    R: ReaderRepository,
{
    let reader = Reader {
        account: account.to_string(),
        name: name.to_string(),
        hash_pwd: str_hash(pwd),
    };
    match reader_repo.add(reader) {
        Ok(()) => Ok(()),
        Err(AddReaderError::AccountAlreadyExists) => Err(SignUpRes::AccountAlreadyExists),
        Err(AddReaderError::RepositoryLockError | AddReaderError::IOError(_)) => {
            Err(SignUpRes::RepositoryError)
        }
    }
}

pub fn librarian_sign_up<L>(
    librarian_repo: &L,
    name: &str,
    account: &str,
    pwd: &str,
) -> Result<(), SignUpRes>
where
    L: LibrarianRepository,
{
    let librarian = Librarian {
        account: account.to_string(),
        name: name.to_string(),
        hash_pwd: str_hash(pwd),
    };
    match librarian_repo.add(librarian) {
        Ok(()) => Ok(()),
        Err(AddLibrarianError::AccountAlreadyExists) => Err(SignUpRes::AccountAlreadyExists),
        Err(AddLibrarianError::RepositoryLockError | AddLibrarianError::IOError(_)) => {
            Err(SignUpRes::RepositoryError)
        }
    }
}

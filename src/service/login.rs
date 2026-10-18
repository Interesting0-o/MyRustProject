use crate::{
    BackendError, Librarian, Reader, UnitOfWork,
    adapter::{librarian_repo::FindLibrarianError, reader_repo::FindReaderError},
};

pub enum LoginError {
    AccountNotFound,
    PasswordError,
    /// 查询仓储失败（IO、序列化、锁中毒等）。
    RepositoryError(BackendError),
}

pub fn reader_login<U: UnitOfWork>(
    uow: &U,
    account: &str,
    pwd: &str,
) -> Result<Reader, LoginError> {
    let repo_group = uow.repositories();
    let reader_repo = repo_group.reader();
    match reader_repo.find_reader_by_account(account) {
        Ok(r) if r.hash_pwd == str_hash(pwd) => Ok(r),
        Ok(_) => Err(LoginError::PasswordError),
        Err(FindReaderError::NoResult) => Err(LoginError::AccountNotFound),
        Err(FindReaderError::Backend(e)) => Err(LoginError::RepositoryError(e)),
    }
}

pub fn librarian_login<U: UnitOfWork>(
    uow: &U,
    account: &str,
    pwd: &str,
) -> Result<Librarian, LoginError> {
    let repo_group = uow.repositories();
    let librarian_repo = repo_group.librarian();
    match librarian_repo.find_librarian_by_account(account) {
        Ok(l) if l.hash_pwd == str_hash(pwd) => Ok(l),
        Ok(_) => Err(LoginError::PasswordError),
        Err(FindLibrarianError::NoResult) => Err(LoginError::AccountNotFound),
        Err(FindLibrarianError::Backend(e)) => Err(LoginError::RepositoryError(e)),
    }
}

/// 把字符串散列成一个 64 位整数，用来存密码摘要。
///
/// 采用 FNV-1a：实现简单、分布均匀，且不需要额外依赖。
/// 注意这只是在「散列」而不是加密——真实系统存密码应该用带盐的 KDF（如 Argon2）。
pub fn str_hash(s: &str) -> u64 {
    const OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;

    s.as_bytes().iter().fold(OFFSET_BASIS, |hash, &byte| {
        (hash ^ byte as u64).wrapping_mul(PRIME)
    })
}

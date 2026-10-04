use crate::{
    Librarian, Reader,
    adapter::{librarian_repo::LibrarianRepository, reader_repo::ReaderRepository},
};

pub enum LoginError {
    AccountNotFound,
    PasswordError,
}

pub fn reader_login<R: ReaderRepository>(
    reader_repo: &R,
    account: &str,
    pwd: &str,
) -> Result<Reader, LoginError> {
    if let Some(r) = reader_repo.find_reader_by_account(account) {
        if r.hash_pwd == str_hash(pwd) {
            Ok(r)
        } else {
            Err(LoginError::PasswordError)
        }
    } else {
        Err(LoginError::AccountNotFound)
    }
}

pub fn librarian_login<L: LibrarianRepository>(
    librarian_repo: &L,
    account: &str,
    pwd: &str,
) -> Result<Librarian, LoginError> {
    if let Some(l) = librarian_repo.find_librarian_by_account(account) {
        if l.hash_pwd == str_hash(pwd) {
            Ok(l)
        } else {
            Err(LoginError::PasswordError)
        }
    } else {
        Err(LoginError::AccountNotFound)
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

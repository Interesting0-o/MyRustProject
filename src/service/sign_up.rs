use crate::{
    AddLibrarianError, AddReaderError, AsyncUnitOfWork, BackendError, Librarian, Reader,
    UnitOfWork, service::login::str_hash,
};

pub enum SignUpRes {
    AccountAlreadyExists,
    /// 写入仓储失败（IO、序列化、锁中毒等）。
    RepositoryError(BackendError),
}

pub fn reader_sign_up<U: UnitOfWork>(
    uow: &U,
    name: &str,
    account: &str,
    pwd: &str,
) -> Result<(), SignUpRes> {
    let reader = Reader {
        account: account.to_string(),
        name: name.to_string(),
        hash_pwd: str_hash(pwd),
    };
    let repo_group = uow.repositories();
    let reader_repo = repo_group.reader();
    match reader_repo.add(reader) {
        Ok(()) => Ok(()),
        Err(AddReaderError::AccountAlreadyExists) => Err(SignUpRes::AccountAlreadyExists),
        Err(AddReaderError::Backend(e)) => Err(SignUpRes::RepositoryError(e)),
    }
}

// 异步注册
pub async fn async_reader_sign_up<U: AsyncUnitOfWork>(
    uow: &mut U,
    name: &str,
    account: &str,
    pwd: &str,
) -> Result<(), SignUpRes> {
    let reader = Reader {
        account: account.to_string(),
        name: name.to_string(),
        hash_pwd: str_hash(pwd),
    };

    let mut repo_group = uow
        .repositories()
        .await
        .map_err(|e| SignUpRes::RepositoryError(e))?;
    match repo_group.reader().add(reader).await {
        Ok(()) => Ok(()),
        Err(AddReaderError::AccountAlreadyExists) => Err(SignUpRes::AccountAlreadyExists),
        Err(AddReaderError::Backend(e)) => Err(SignUpRes::RepositoryError(e)),
    }
}

pub fn librarian_sign_up<U: UnitOfWork>(
    uow: &U,
    name: &str,
    account: &str,
    pwd: &str,
) -> Result<(), SignUpRes> {
    let librarian = Librarian {
        account: account.to_string(),
        name: name.to_string(),
        hash_pwd: str_hash(pwd),
    };
    let repo_group = uow.repositories();
    let librarian_repo = repo_group.librarian();
    match librarian_repo.add(librarian) {
        Ok(()) => Ok(()),
        Err(AddLibrarianError::AccountAlreadyExists) => Err(SignUpRes::AccountAlreadyExists),
        Err(AddLibrarianError::Backend(e)) => Err(SignUpRes::RepositoryError(e)),
    }
}

//异步注册

pub async fn async_librarain_sign_up<U: AsyncUnitOfWork>(
    uow: &mut U,
    name: &str,
    account: &str,
    pwd: &str,
) -> Result<(), SignUpRes> {
    let librarian = Librarian {
        account: account.to_string(),
        name: name.to_string(),
        hash_pwd: str_hash(pwd),
    };

    let mut repo_group = uow
        .repositories()
        .await
        .map_err(|e| SignUpRes::RepositoryError(e))?;
    match repo_group.librarian().add(librarian).await {
        Ok(()) => Ok(()),
        Err(AddLibrarianError::AccountAlreadyExists) => Err(SignUpRes::AccountAlreadyExists),
        Err(AddLibrarianError::Backend(e)) => Err(SignUpRes::RepositoryError(e)),
    }
}

use crate::{
    BackendError, FindReaderError, Reader, UnitOfWork,
    adapter::reader_repo::UpdateReaderError as RepositoryUpdateReaderError,
    service::login::str_hash,
};

pub enum UpdateReaderProfileError {
    NonExistedAccount,
    PasswordError,
    NoChanges,
    RepositoryError(BackendError),
}

pub fn update_reader<U: UnitOfWork>(
    uow: &U,
    account: &str,
    name: Option<&str>,
    current_pwd: Option<&str>,
    new_pwd: Option<&str>,
) -> Result<(), UpdateReaderProfileError> {
    let repo_group = uow.repositories();
    let reader_repo = repo_group.reader();

    let reader = match reader_repo.find_reader_by_account(account) {
        Ok(reader) => reader,
        Err(FindReaderError::NoResult) => return Err(UpdateReaderProfileError::NonExistedAccount),
        Err(FindReaderError::Backend(e)) => {
            return Err(UpdateReaderProfileError::RepositoryError(e));
        }
    };

    let updated_name = name
        .map(str::to_string)
        .unwrap_or_else(|| reader.name.clone());
    let updated_hash_pwd = match new_pwd {
        Some(new_pwd) => {
            if reader.hash_pwd != str_hash(current_pwd.unwrap_or_default()) {
                return Err(UpdateReaderProfileError::PasswordError);
            }
            str_hash(new_pwd)
        }
        None => reader.hash_pwd,
    };

    if updated_name == reader.name && updated_hash_pwd == reader.hash_pwd {
        return Err(UpdateReaderProfileError::NoChanges);
    }

    reader_repo
        .update(Reader {
            account: reader.account,
            name: updated_name,
            hash_pwd: updated_hash_pwd,
        })
        .map_err(|e| match e {
            RepositoryUpdateReaderError::NonExistedAccount => {
                UpdateReaderProfileError::NonExistedAccount
            }
            RepositoryUpdateReaderError::Backend(e) => UpdateReaderProfileError::RepositoryError(e),
        })
}

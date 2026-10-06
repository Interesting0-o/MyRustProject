use crate::{
    AddLibrarianError, AddReaderError, Librarian, LibrarianRepository, Reader, ReaderRepository,
    service::login::str_hash,
};

pub enum SignUpRes {
    AccountAlreadyExists,
}

pub fn reader_sign_up<R>(
    reader_repo: &mut R,
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
    if let Err(AddReaderError::AccountAlreadyExists) = reader_repo.add(reader) {
        Err(SignUpRes::AccountAlreadyExists)
    } else {
        Ok(())
    }
}

pub fn librarian_sign_up<L>(
    librarian_repo: &mut L,
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
    if let Err(AddLibrarianError::AccountAlreadyExists) = librarian_repo.add(librarian) {
        Err(SignUpRes::AccountAlreadyExists)
    } else {
        Ok(())
    }
}

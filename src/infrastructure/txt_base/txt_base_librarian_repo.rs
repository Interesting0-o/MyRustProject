use crate::{
    AddLibrarianError, Librarian, LibrarianRepository, UpdateLibrarianError,
    adapter::librarian_repo::FindLibrarianError,
    utils::{formatter::format_librarian_for_storage, generator::generator_librarian_from_str},
};
use std::io::Write;
use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader},
    sync::RwLock,
};

use super::TXTBaseNewError;

pub struct TxtBaseLibrarianRepository {
    librarians: RwLock<Vec<Librarian>>,
    repo_path: String,
}

impl TxtBaseLibrarianRepository {
    pub fn new(path: &str) -> Result<Self, TXTBaseNewError> {
        if path.trim().is_empty() {
            return Err(TXTBaseNewError::PathIsEmpty);
        }

        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
            .map_err(TXTBaseNewError::OpenFailed)?;
        let librarians = BufReader::new(f)
            .lines()
            .map_while(Result::ok)
            .filter_map(|l| generator_librarian_from_str(&l).ok())
            .collect();
        Ok(Self {
            librarians: RwLock::new(librarians),
            repo_path: path.to_string(),
        })
    }
}

impl LibrarianRepository for TxtBaseLibrarianRepository {
    fn add(&self, librarian: Librarian) -> Result<(), AddLibrarianError> {
        let mut librarians = self
            .librarians
            .write()
            .map_err(|_| AddLibrarianError::RepositoryLockError)?;
        if librarians.iter().any(|l| l.account == librarian.account) {
            Err(AddLibrarianError::AccountAlreadyExists)
        } else {
            let deal_err = |e| AddLibrarianError::IOError(e);
            librarians.push(librarian);
            let tmp_path = format!("{}.tmp", self.repo_path);
            let mut file = File::create(&tmp_path).map_err(deal_err)?;
            for l in librarians.iter() {
                write!(file, "{}", format_librarian_for_storage(l)).map_err(deal_err)?;
            }
            file.sync_all().map_err(deal_err)?;
            fs::rename(&tmp_path, &self.repo_path).map_err(deal_err)?;
            Ok(())
        }
    }

    fn update(&self, librarian: Librarian) -> Result<(), UpdateLibrarianError> {
        let mut librarians = self
            .librarians
            .write()
            .map_err(|_| UpdateLibrarianError::RepositoryLockError)?;

        if let Some(l) = librarians
            .iter_mut()
            .find(|l| l.account == librarian.account)
        {
            *l = librarian;
            let deal_err = |e| UpdateLibrarianError::IOError(e);
            let tmp_path = format!("{}.tmp", self.repo_path);
            let mut file = File::create(&tmp_path).map_err(deal_err)?;
            for l in librarians.iter() {
                write!(file, "{}", format_librarian_for_storage(l)).map_err(deal_err)?;
            }
            file.sync_all().map_err(deal_err)?;
            fs::rename(&tmp_path, &self.repo_path).map_err(deal_err)?;
            Ok(())
        } else {
            Err(UpdateLibrarianError::NonExistedAccount)
        }
    }
    fn find_librarian_by_account(&self, account: &str) -> Result<Librarian, FindLibrarianError> {
        let librarians = self
            .librarians
            .read()
            .map_err(|_| FindLibrarianError::RepositoryLockError)?;
        if let Some(l) = librarians.iter().find(|l| l.account == account).cloned() {
            Ok(l)
        } else {
            Err(FindLibrarianError::NoResult)
        }
    }
    fn find_librarian_by_name(&self, name: &str) -> Result<Vec<Librarian>, FindLibrarianError> {
        let librarians = self
            .librarians
            .read()
            .map_err(|_| FindLibrarianError::RepositoryLockError)?;
        let query_res: Vec<Librarian> = librarians
            .iter()
            .filter(|l| l.name == name)
            .cloned()
            .collect();

        if query_res.is_empty() {
            Err(FindLibrarianError::NoResult)
        } else {
            Ok(query_res)
        }
    }
}

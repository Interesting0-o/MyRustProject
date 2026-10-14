use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Write},
    sync::RwLock,
};

use crate::{
    AddLibrarianError, FindLibrarianError, Librarian, LibrarianRepository, UpdateLibrarianError,
    infrastructure::{json_base::JSONBaseNewError, lock_poisoned},
};

pub struct JSONBaseLibrarianRepository {
    librarians: RwLock<Vec<Librarian>>,
    repo_path: String,
}

impl JSONBaseLibrarianRepository {
    pub fn new(path: &str) -> Result<Self, JSONBaseNewError> {
        if path.trim().is_empty() {
            return Err(JSONBaseNewError::PathIsEmpty);
        }

        let file = File::open(path).map_err(JSONBaseNewError::OpenFailed)?;
        let librarians = serde_json::from_reader(BufReader::new(file))
            .map_err(|_| JSONBaseNewError::ParseFailed)?;
        Ok(Self {
            librarians: RwLock::new(librarians),
            repo_path: path.to_string(),
        })
    }
}

impl LibrarianRepository for JSONBaseLibrarianRepository {
    fn add(&self, librarian: Librarian) -> Result<(), AddLibrarianError> {
        let mut librarians = self
            .librarians
            .write()
            .map_err(|_| AddLibrarianError::Backend(lock_poisoned()))?;
        if librarians.iter().any(|l| l.account == librarian.account) {
            return Err(AddLibrarianError::AccountAlreadyExists);
        }
        librarians.push(librarian);

        let deal_err = |e| AddLibrarianError::Backend(Box::new(e));
        let tmp_path = format!("{}.tmp", self.repo_path);
        let file = File::create(&tmp_path).map_err(deal_err)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &*librarians)
            .map_err(|e| AddLibrarianError::Backend(Box::new(e)))?;
        writer.flush().map_err(deal_err)?;
        writer.get_ref().sync_all().map_err(deal_err)?;
        fs::rename(tmp_path, &self.repo_path).map_err(deal_err)?;
        Ok(())
    }

    fn update(&self, librarian: Librarian) -> Result<(), UpdateLibrarianError> {
        let mut librarians = self
            .librarians
            .write()
            .map_err(|_| UpdateLibrarianError::Backend(lock_poisoned()))?;

        if let Some(l) = librarians
            .iter_mut()
            .find(|l| l.account == librarian.account)
        {
            *l = librarian;

            let deal_err = |e| UpdateLibrarianError::Backend(Box::new(e));
            let tmp_path = format!("{}.tmp", self.repo_path);
            let file = File::create(&tmp_path).map_err(deal_err)?;
            let mut writer = BufWriter::new(file);
            serde_json::to_writer(&mut writer, &*librarians)
                .map_err(|e| UpdateLibrarianError::Backend(Box::new(e)))?;
            writer.flush().map_err(deal_err)?;
            writer.get_ref().sync_all().map_err(deal_err)?;
            fs::rename(tmp_path, &self.repo_path).map_err(deal_err)?;
            Ok(())
        } else {
            Err(UpdateLibrarianError::NonExistedAccount)
        }
    }

    fn find_librarian_by_account(&self, account: &str) -> Result<Librarian, FindLibrarianError> {
        let librarians = self
            .librarians
            .read()
            .map_err(|_| FindLibrarianError::Backend(lock_poisoned()))?;
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
            .map_err(|_| FindLibrarianError::Backend(lock_poisoned()))?;
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

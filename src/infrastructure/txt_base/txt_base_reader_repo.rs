use crate::{
    AddReaderError, Reader, ReaderRepository, UpdateReaderError,
    adapter::reader_repo::FindReaderError,
    utils::{formatter::format_reader_for_storage, generator::generator_reader_from_str},
};
use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    sync::RwLock,
};

use super::TXTBaseNewError;

pub struct TXTBaseReaderRepository {
    readers: RwLock<Vec<Reader>>,
    repo_path: String,
}

impl TXTBaseReaderRepository {
    /// 构造仓储，并把 `resource/readers.txt` 里已有的读者读进来。
    ///
    /// 构造不属于 [`ReaderRepository`] 契约，由各个实现自行提供。
    pub fn new(repo_path: &str) -> Result<Self, TXTBaseNewError> {
        if repo_path.trim().is_empty() {
            return Err(TXTBaseNewError::PathIsEmpty);
        }

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(false)
            .create(true)
            .open(repo_path)
            .map_err(TXTBaseNewError::OpenFailed)?;

        let readers: Vec<Reader> = BufReader::new(file)
            .lines()
            .map_while(Result::ok)
            .filter_map(|l| generator_reader_from_str(&l).ok())
            .collect();
        Ok(Self {
            repo_path: repo_path.to_string(),
            readers: RwLock::new(readers),
        })
    }
}
impl ReaderRepository for TXTBaseReaderRepository {
    fn add(&self, reader: Reader) -> Result<(), AddReaderError> {
        let mut readers = self
            .readers
            .write()
            .map_err(|_| AddReaderError::RepositoryLockError)?;

        if readers.iter().any(|r| r.account == reader.account) {
            Err(AddReaderError::AccountAlreadyExists)
        } else {
            readers.push(reader);
            let deal_err = |e| AddReaderError::IOError(e);
            let tmp_path = format!("{}.tmp", self.repo_path);
            let mut file = File::create(&tmp_path).map_err(deal_err)?;
            for r in readers.iter() {
                write!(file, "{}", format_reader_for_storage(r)).map_err(deal_err)?;
            }
            file.sync_all().map_err(deal_err)?;
            fs::rename(tmp_path, &self.repo_path).map_err(deal_err)?;
            Ok(())
        }
    }

    fn update(&self, reader: Reader) -> Result<(), UpdateReaderError> {
        let mut readers = self
            .readers
            .write()
            .map_err(|_| UpdateReaderError::RepositoryLockError)?;

        if let Some(r) = readers.iter_mut().find(|r| r.account == reader.account) {
            *r = reader;
            let deal_err = |e| UpdateReaderError::IOError(e);

            let tmp_path = format!("{}.tmp", self.repo_path);
            let mut file = File::create(&tmp_path).map_err(deal_err)?;
            for rder in readers.iter() {
                write!(file, "{}", format_reader_for_storage(rder)).map_err(deal_err)?;
            }
            file.sync_all().map_err(deal_err)?;
            fs::rename(tmp_path, &self.repo_path).map_err(deal_err)?;
            Ok(())
        } else {
            Err(UpdateReaderError::NonExistedAccount)
        }
    }
    fn find_reader_by_name(&self, name: &str) -> Result<Vec<Reader>, FindReaderError> {
        let readers = self
            .readers
            .read()
            .map_err(|_| FindReaderError::RepositoryLockError)?;
        let query_res: Vec<Reader> = readers.iter().filter(|r| r.name == name).cloned().collect();
        if query_res.is_empty() {
            Err(FindReaderError::NoResult)
        } else {
            Ok(query_res)
        }
    }
    fn find_reader_by_account(&self, account: &str) -> Result<Reader, FindReaderError> {
        let readers = self
            .readers
            .read()
            .map_err(|_| FindReaderError::RepositoryLockError)?;
        if let Some(reader) = readers.iter().find(|r| r.account == account).cloned() {
            Ok(reader)
        } else {
            Err(FindReaderError::NoResult)
        }
    }
}

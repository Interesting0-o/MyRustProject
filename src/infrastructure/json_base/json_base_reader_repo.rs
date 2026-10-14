use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Write},
    sync::RwLock,
};

use crate::{
    AddReaderError, FindReaderError, Reader, ReaderRepository, UpdateReaderError,
    infrastructure::{json_base::JSONBaseNewError, lock_poisoned},
};

pub struct JSONBaseReadderRepository {
    readers: RwLock<Vec<Reader>>,
    repo_path: String,
}

impl JSONBaseReadderRepository {
    pub fn new(path: &str) -> Result<Self, JSONBaseNewError> {
        if path.trim().is_empty() {
            return Err(JSONBaseNewError::PathIsEmpty);
        }
        let file = File::open(path).map_err(JSONBaseNewError::OpenFailed)?;
        let readers = serde_json::from_reader(BufReader::new(file))
            .map_err(|_| JSONBaseNewError::ParseFailed)?;
        Ok(Self {
            readers: RwLock::new(readers),
            repo_path: path.to_string(),
        })
    }
}

impl ReaderRepository for JSONBaseReadderRepository {
    fn add(&self, reader: Reader) -> Result<(), AddReaderError> {
        let mut readers = self
            .readers
            .write()
            .map_err(|_| AddReaderError::Backend(lock_poisoned()))?;
        if readers.iter().any(|r| r.account == reader.account) {
            Err(AddReaderError::AccountAlreadyExists)
        } else {
            readers.push(reader);

            let tmp_path = format!("{}.tmp", self.repo_path);
            let deal_err = |e| AddReaderError::Backend(Box::new(e));
            let file = File::create(&tmp_path).map_err(deal_err)?;
            let mut writer = BufWriter::new(file);
            serde_json::to_writer(&mut writer, &*readers)
                .map_err(|e| AddReaderError::Backend(Box::new(e)))?;
            writer.flush().map_err(deal_err)?;
            writer.get_ref().sync_all().map_err(deal_err)?;
            fs::rename(&tmp_path, &self.repo_path).map_err(deal_err)?;

            Ok(())
        }
    }

    fn update(&self, reader: Reader) -> Result<(), UpdateReaderError> {
        let mut readers = self
            .readers
            .write()
            .map_err(|_| UpdateReaderError::Backend(lock_poisoned()))?;
        if let Some(r) = readers
            .iter_mut()
            .find(|tmp_r| tmp_r.account == reader.account)
        {
            *r = reader;

            let tmp_path = format!("{}.tmp", self.repo_path);
            let deal_err = |e| UpdateReaderError::Backend(Box::new(e));
            let file = File::create(&tmp_path).map_err(deal_err)?;
            let mut writer = BufWriter::new(file);
            serde_json::to_writer(&mut writer, &*readers)
                .map_err(|e| UpdateReaderError::Backend(Box::new(e)))?;
            writer.flush().map_err(deal_err)?;
            writer.get_ref().sync_all().map_err(deal_err)?;
            fs::rename(&tmp_path, &self.repo_path).map_err(deal_err)?;
            Ok(())
        } else {
            Err(UpdateReaderError::NonExistedAccount)
        }
    }

    fn find_reader_by_name(&self, name: &str) -> Result<Vec<Reader>, FindReaderError> {
        let readers = self
            .readers
            .read()
            .map_err(|_| FindReaderError::Backend(lock_poisoned()))?;
        let query_res = readers
            .iter()
            .filter(|r| r.name == name)
            .cloned()
            .collect::<Vec<Reader>>();
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
            .map_err(|_| FindReaderError::Backend(lock_poisoned()))?;
        if let Some(reader) = readers.iter().find(|r| r.account == account).cloned() {
            Ok(reader)
        } else {
            Err(FindReaderError::NoResult)
        }
    }
}

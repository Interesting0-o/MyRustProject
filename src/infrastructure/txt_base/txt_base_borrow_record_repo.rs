use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    sync::{Mutex, RwLock},
};

use crate::{
    BorrowRecord, BorrowRecordRepository, RemoveBorrowRecordError, TXTBaseNewError,
    adapter::borrow_record_repo::{AddBorrowRecordError, FindBorrowRecordError},
    utils::{
        formatter::format_borrow_record_for_storage, generator::generator_borrow_record_from_str,
    },
};

pub struct TXTBaseBorrowRecordRepository {
    records: RwLock<Vec<BorrowRecord>>,
    pk_record: Mutex<u64>,
    repo_path: String,
}

impl TXTBaseBorrowRecordRepository {
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
        let records: Vec<BorrowRecord> = BufReader::new(f)
            .lines()
            .map_while(Result::ok)
            .filter_map(|br| generator_borrow_record_from_str(&br).ok())
            .collect();

        let pk_record = records
            .iter()
            .filter_map(|r| r.br_id.strip_prefix("BR")?.parse::<u64>().ok())
            .max()
            .unwrap_or(0);

        Ok(Self {
            records: RwLock::new(records),
            pk_record: Mutex::new(pk_record),
            repo_path: path.to_string(),
        })
    }
}

impl BorrowRecordRepository for TXTBaseBorrowRecordRepository {
    fn add(&self, account: &str, bid: &str) -> Result<(), AddBorrowRecordError> {
        let mut records = self
            .records
            .write()
            .map_err(|_| AddBorrowRecordError::RepositoryLockError)?;
        let mut pk_record = self
            .pk_record
            .lock()
            .map_err(|_| AddBorrowRecordError::RepositoryLockError)?;

        *pk_record += 1;
        let br_id = format!("BR{}", *pk_record);
        records.push(BorrowRecord {
            br_id,
            account: account.to_string(),
            bid: bid.to_string(),
        });
        let tmp_path = format!("{}.tmp", self.repo_path);
        let deal_err = |e| AddBorrowRecordError::IOError(e);
        let mut file = File::create(&tmp_path).map_err(deal_err)?;
        for br in records.iter() {
            writeln!(file, "{}", format_borrow_record_for_storage(br)).map_err(deal_err)?;
        }
        file.sync_all().map_err(deal_err)?;
        fs::rename(tmp_path, &self.repo_path).map_err(deal_err)?;
        Ok(())
    }

    fn remove(&self, br_id: &str) -> Result<(), RemoveBorrowRecordError> {
        let mut records = self
            .records
            .write()
            .map_err(|_| RemoveBorrowRecordError::RepositoryLockError)?;

        let Some(index) = records.iter().position(|r| r.br_id == br_id) else {
            return Err(RemoveBorrowRecordError::NonExistedBRID);
        };
        records.remove(index);

        let tmp_path = format!("{}.tmp", self.repo_path);
        let deal_err = |e| RemoveBorrowRecordError::IOError(e);
        let mut file = File::create(&tmp_path).map_err(deal_err)?;
        for br in records.iter() {
            writeln!(file, "{}", format_borrow_record_for_storage(br)).map_err(deal_err)?;
        }
        file.sync_all().map_err(deal_err)?;
        fs::rename(&tmp_path, &self.repo_path).map_err(deal_err)?;
        Ok(())
    }

    fn find_record_by_brid(&self, br_id: &str) -> Result<BorrowRecord, FindBorrowRecordError> {
        let records = self
            .records
            .read()
            .map_err(|_| FindBorrowRecordError::RepositoryLockError)?;
        if let Some(br) = records.iter().find(|r| r.br_id == br_id).cloned() {
            Ok(br)
        } else {
            Err(FindBorrowRecordError::NoResult)
        }
    }
    fn find_records_by_bid(&self, bid: &str) -> Result<Vec<BorrowRecord>, FindBorrowRecordError> {
        let records = self
            .records
            .read()
            .map_err(|_| FindBorrowRecordError::RepositoryLockError)?;
        let query_res: Vec<BorrowRecord> =
            records.iter().filter(|r| r.bid == bid).cloned().collect();

        if query_res.is_empty() {
            Err(FindBorrowRecordError::NoResult)
        } else {
            Ok(query_res)
        }
    }
    fn find_records_by_account(
        &self,
        account: &str,
    ) -> Result<Vec<BorrowRecord>, FindBorrowRecordError> {
        let records = self
            .records
            .read()
            .map_err(|_| FindBorrowRecordError::RepositoryLockError)?;
        let query_res: Vec<BorrowRecord> = records
            .iter()
            .filter(|r| r.account == account)
            .cloned()
            .collect();

        if query_res.is_empty() {
            Err(FindBorrowRecordError::NoResult)
        } else {
            Ok(query_res)
        }
    }
    fn find_records_by_account_and_bid(
        &self,
        account: &str,
        bid: &str,
    ) -> Result<Vec<BorrowRecord>, FindBorrowRecordError> {
        let records = self
            .records
            .read()
            .map_err(|_| FindBorrowRecordError::RepositoryLockError)?;
        let query_res: Vec<BorrowRecord> = records
            .iter()
            .filter(|r| r.account == account && r.bid == bid)
            .cloned()
            .collect();

        if query_res.is_empty() {
            Err(FindBorrowRecordError::NoResult)
        } else {
            Ok(query_res)
        }
    }
}

use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Write},
    sync::{Mutex, RwLock},
};

use crate::{
    BorrowRecord, BorrowRecordRepository, RemoveBorrowRecordError,
    adapter::borrow_record_repo::{AddBorrowRecordError, FindBorrowRecordError},
    infrastructure::{json_base::JSONBaseNewError, lock_poisoned},
};

pub struct JSONBaseBorrowRecordRepository {
    records: RwLock<Vec<BorrowRecord>>,
    repo_path: String,
    pk_record: Mutex<u64>,
}

impl JSONBaseBorrowRecordRepository {
    pub fn new(path: &str) -> Result<Self, JSONBaseNewError> {
        if path.trim().is_empty() {
            return Err(JSONBaseNewError::PathIsEmpty);
        }

        let file = File::open(path).map_err(JSONBaseNewError::OpenFailed)?;
        let records: Vec<BorrowRecord> = serde_json::from_reader(BufReader::new(file))
            .map_err(|_| JSONBaseNewError::ParseFailed)?;

        let pk_record = records
            .iter()
            .filter_map(|r| r.br_id.strip_prefix("BR")?.parse::<u64>().ok())
            .max()
            .unwrap_or(0);

        Ok(Self {
            records: RwLock::new(records),
            repo_path: path.to_string(),
            pk_record: Mutex::new(pk_record),
        })
    }
}

impl BorrowRecordRepository for JSONBaseBorrowRecordRepository {
    fn add(&self, account: &str, bid: &str) -> Result<(), AddBorrowRecordError> {
        let mut records = self
            .records
            .write()
            .map_err(|_| AddBorrowRecordError::Backend(lock_poisoned()))?;
        let mut pk_record = self
            .pk_record
            .lock()
            .map_err(|_| AddBorrowRecordError::Backend(lock_poisoned()))?;

        *pk_record += 1;
        let br_id = format!("BR{}", *pk_record);
        records.push(BorrowRecord {
            br_id,
            bid: bid.to_string(),
            account: account.to_string(),
        });

        let deal_err = |e| AddBorrowRecordError::Backend(Box::new(e));
        let tmp_path = format!("{}.tmp", self.repo_path);
        let file = File::create(&tmp_path).map_err(deal_err)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &*records)
            .map_err(|e| AddBorrowRecordError::Backend(Box::new(e)))?;
        writer.flush().map_err(deal_err)?;
        writer.get_ref().sync_all().map_err(deal_err)?;
        fs::rename(tmp_path, &self.repo_path).map_err(deal_err)?;
        Ok(())
    }

    fn remove(&self, br_id: &str) -> Result<(), RemoveBorrowRecordError> {
        let mut records = self
            .records
            .write()
            .map_err(|_| RemoveBorrowRecordError::Backend(lock_poisoned()))?;

        let Some(index) = records.iter().position(|r| r.br_id == br_id) else {
            return Err(RemoveBorrowRecordError::NonExistedBRID);
        };
        records.remove(index);

        let deal_err = |e| RemoveBorrowRecordError::Backend(Box::new(e));
        let tmp_path = format!("{}.tmp", self.repo_path);
        let file = File::create(&tmp_path).map_err(deal_err)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &*records)
            .map_err(|e| RemoveBorrowRecordError::Backend(Box::new(e)))?;
        writer.flush().map_err(deal_err)?;
        writer.get_ref().sync_all().map_err(deal_err)?;
        fs::rename(tmp_path, &self.repo_path).map_err(deal_err)?;
        Ok(())
    }

    fn find_record_by_brid(&self, br_id: &str) -> Result<BorrowRecord, FindBorrowRecordError> {
        let records = self
            .records
            .read()
            .map_err(|_| FindBorrowRecordError::Backend(lock_poisoned()))?;
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
            .map_err(|_| FindBorrowRecordError::Backend(lock_poisoned()))?;
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
            .map_err(|_| FindBorrowRecordError::Backend(lock_poisoned()))?;
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
            .map_err(|_| FindBorrowRecordError::Backend(lock_poisoned()))?;
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

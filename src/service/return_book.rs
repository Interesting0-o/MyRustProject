use crate::{
    Book, BookRepository, BorrowRecordRepository, FindBookError,
    adapter::{
        book_repo::UpdateBookError,
        borrow_record_repo::{FindBorrowRecordError, RemoveBorrowRecordError},
    },
};

pub enum ReturnBookError {
    NonExistBID,
    NotFoundRecord,
    /// 查询图书仓储失败（锁中毒、读取持久化数据出错等）。
    RepositoryError(FindBookError),
    /// 更新图书失败（锁中毒、写持久化数据出错等）。
    UpdateBookError(UpdateBookError),
    /// 查询借阅记录失败（锁中毒、读取持久化数据出错等）。
    FindBorrowRecordError(FindBorrowRecordError),
    /// 删除借阅记录失败（锁中毒、写持久化数据出错等）。
    RemoveBorrowRecordError(RemoveBorrowRecordError),
}

pub fn return_book<B, BR>(
    book_repo: &B,
    borrow_record_repo: &BR,
    bid: &str,
    account: &str,
) -> Result<(), ReturnBookError>
where
    B: BookRepository,
    BR: BorrowRecordRepository,
{
    let book = match book_repo.find_book_by_bid(bid) {
        Ok(book) => book,
        Err(FindBookError::NoResult) => return Err(ReturnBookError::NonExistBID),
        Err(e) => return Err(ReturnBookError::RepositoryError(e)),
    };

    let record = match borrow_record_repo.find_records_by_account_and_bid(account, bid) {
        Ok(records) => match records.into_iter().next() {
            Some(record) => record,
            None => return Err(ReturnBookError::NotFoundRecord),
        },
        Err(FindBorrowRecordError::NoResult) => return Err(ReturnBookError::NotFoundRecord),
        Err(e) => return Err(ReturnBookError::FindBorrowRecordError(e)),
    };

    book_repo
        .update(Book {
            num: book.num + 1,
            ..book
        })
        .map_err(ReturnBookError::UpdateBookError)?;

    borrow_record_repo
        .remove(&record.br_id)
        .map_err(ReturnBookError::RemoveBorrowRecordError)?;

    Ok(())
}

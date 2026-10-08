use crate::{Book, BookRepository, BorrowRecordRepository, FindBookError};

pub enum ReturnBookError {
    NonExistBID,
    NotFoundRecord,
    /// 查询图书仓储失败（锁中毒、读取持久化数据出错等）。
    RepositoryError(FindBookError),
}

pub fn return_book<B, BR>(
    book_repo: &mut B,
    borrow_record_repo: &mut BR,
    bid: &str,
    account: &str,
) -> Result<(), ReturnBookError>
where
    B: BookRepository,
    BR: BorrowRecordRepository,
{
    match book_repo.find_book_by_bid(bid) {
        Ok(_) => {
            if let Some(br) = borrow_record_repo
                .find_records_by_bid(bid)
                .iter()
                .find(|b| b.bid == bid && b.account == account)
            {
                let book = book_repo
                    .find_book_by_bid(&br.bid)
                    .map_err(ReturnBookError::RepositoryError)?;
                let res = Book {
                    num: book.num + 1,
                    ..book
                };
                book_repo
                    .update(res)
                    .expect("book 刚由 bid 查到且未被改动，update 不可能失败");
                borrow_record_repo
                    .remove(&br.br_id)
                    .expect("记录刚由 find_records_by_bid 查到，remove 不可能失败");
                Ok(())
            } else {
                Err(ReturnBookError::NotFoundRecord)
            }
        }
        Err(FindBookError::NoResult) => Err(ReturnBookError::NonExistBID),
        Err(e) => Err(ReturnBookError::RepositoryError(e)),
    }
}

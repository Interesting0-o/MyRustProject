use crate::{Book, BookRepository, BorrowRecordRepository, FindBookError};
pub enum BorrowBookError {
    NonExistBID,
    BookNumIsZero,
    /// 查询图书仓储失败（锁中毒、读取持久化数据出错等）。
    RepositoryError(FindBookError),
}

pub fn borrow_book<B, BR>(
    book_repo: &mut B,
    borrow_record_repo: &mut BR,
    account: &str,
    bid: &str,
) -> Result<(), BorrowBookError>
where
    B: BookRepository,
    BR: BorrowRecordRepository,
{
    match book_repo.find_book_by_bid(bid) {
        Ok(book) => {
            if book.num == 0 {
                Err(BorrowBookError::BookNumIsZero)
            } else {
                let b = Book {
                    num: book.num - 1,
                    ..book
                };
                // update 必然命中：bid 刚由上面的 find 查到，且这里没有改动 bid
                book_repo
                    .update(b)
                    .expect("book 刚由 bid 查到且未被改动，update 不可能失败");
                borrow_record_repo.add(account, bid);
                Ok(())
            }
        }
        Err(FindBookError::NoResult) => Err(BorrowBookError::NonExistBID),
        Err(e) => Err(BorrowBookError::RepositoryError(e)),
    }
}

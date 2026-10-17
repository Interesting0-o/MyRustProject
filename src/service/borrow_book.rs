use crate::{
    Book, BookRepository, BorrowRecordRepository, FindBookError, UnitOfWork, UnitOfWorkError,
    adapter::{book_repo::UpdateBookError, borrow_record_repo::AddBorrowRecordError},
};

pub enum BorrowBookError {
    NonExistBID,
    BookNumIsZero,
    /// 查询图书仓储失败（锁中毒、读取持久化数据出错等）。
    RepositoryError(FindBookError),
    /// 更新图书失败（锁中毒、写持久化数据出错等）。
    UpdateBookError(UpdateBookError),
    /// 新增借阅记录失败（锁中毒、写持久化数据出错等）。
    AddBorrowRecordError(AddBorrowRecordError),
}

pub fn borrow_book<B, BR>(
    book_repo: &B,
    borrow_record_repo: &BR,
    account: &str,
    bid: &str,
) -> Result<(), BorrowBookError>
where
    B: BookRepository,
    BR: BorrowRecordRepository,
{
    let book = match book_repo.find_book_by_bid(bid) {
        Ok(book) => book,
        Err(FindBookError::NoResult) => return Err(BorrowBookError::NonExistBID),
        Err(e) => return Err(BorrowBookError::RepositoryError(e)),
    };

    if book.num == 0 {
        return Err(BorrowBookError::BookNumIsZero);
    }

    book_repo
        .update(Book {
            num: book.num - 1,
            ..book
        })
        .map_err(BorrowBookError::UpdateBookError)?;

    borrow_record_repo
        .add(account, bid)
        .map_err(BorrowBookError::AddBorrowRecordError)?;

    Ok(())
}

pub fn borrow_bookk<U>(uow: &mut U, account: &str, bid: &str) -> Result<(), BorrowBookError>
where
    U: UnitOfWork,
{
    uow.transaction(|repo_group| {
        let book_repo = repo_group.book();
        let borrow_record_repo = repo_group.borrow_record();

        let book = match book_repo.find_book_by_bid(bid) {
            Ok(book) => book,
            Err(FindBookError::NoResult) => return Err(BorrowBookError::NonExistBID),
            Err(e) => return Err(BorrowBookError::RepositoryError(e)),
        };

        if book.num == 0 {
            return Err(BorrowBookError::BookNumIsZero);
        }

        book_repo
            .update(Book {
                num: book.num - 1,
                ..book
            })
            .map_err(BorrowBookError::UpdateBookError)?;

        borrow_record_repo
            .add(account, bid)
            .map_err(BorrowBookError::AddBorrowRecordError)?;

        Ok(())
    })
    .map_err(|e| match e {
        UnitOfWorkError::Transaction(e) => e,
        UnitOfWorkError::Backend(e) => BorrowBookError::RepositoryError(FindBookError::Backend(e)),
    })
}

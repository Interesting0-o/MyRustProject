use crate::{
    AsyncUnitOfWork, Book, FindBookError, UnitOfWorkError,
    adapter::{book_repo::UpdateBookError, borrow_record_repo::AddBorrowRecordError},
};

#[derive(Debug)]
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

pub async fn async_borrow_book<U: AsyncUnitOfWork>(
    uow: &mut U,
    account: &str,
    bid: &str,
) -> Result<(), BorrowBookError> {
    uow.transaction(|repo_group| {
        let account = account.to_owned();
        let bid = bid.to_owned();

        Box::pin(async move {
            {
                let mut book_repo = repo_group.book();

                let book = match book_repo.find_book_by_bid(&bid).await {
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
                    .await
                    .map_err(BorrowBookError::UpdateBookError)?;
            }

            repo_group
                .borrow_record()
                .add(&account, &bid)
                .await
                .map_err(BorrowBookError::AddBorrowRecordError)?;

            Ok(())
        })
    })
    .await
    .map_err(|e| match e {
        UnitOfWorkError::Transaction(e) => e,
        UnitOfWorkError::Backend(e) => BorrowBookError::RepositoryError(FindBookError::Backend(e)),
    })
}

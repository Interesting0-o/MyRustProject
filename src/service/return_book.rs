use crate::{
    AsyncRepositoryGroup, AsyncUnitOfWork, Book, FindBookError, UnitOfWorkError,
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

pub async fn async_return_book<U: AsyncUnitOfWork>(
    uow: &mut U,
    account: &str,
    bid: &str,
) -> Result<(), ReturnBookError> {
    let account = account.to_owned();
    let bid = bid.to_owned();

    uow.transaction(|repo_group: &mut dyn AsyncRepositoryGroup| {
        Box::pin(async move {
            let book = {
                let mut book_repo = repo_group.book();
                match book_repo.find_book_by_bid(&bid).await {
                    Ok(b) => b,
                    Err(FindBookError::NoResult) => return Err(ReturnBookError::NonExistBID),
                    Err(e) => return Err(ReturnBookError::RepositoryError(e)),
                }
            };

            let record = {
                let mut borrow_record_repo = repo_group.borrow_record();
                match borrow_record_repo
                    .find_records_by_account_and_bid(&account, &bid)
                    .await
                {
                    Ok(records) => match records.into_iter().next() {
                        Some(record) => record,
                        None => return Err(ReturnBookError::NotFoundRecord),
                    },
                    Err(FindBorrowRecordError::NoResult) => {
                        return Err(ReturnBookError::NotFoundRecord);
                    }
                    Err(e) => {
                        return Err(ReturnBookError::FindBorrowRecordError(e));
                    }
                }
            };

            {
                let mut book_repo = repo_group.book();
                book_repo
                    .update(Book {
                        num: book.num + 1,
                        ..book
                    })
                    .await
                    .map_err(ReturnBookError::UpdateBookError)?;
            }

            {
                let mut borrow_record_repo = repo_group.borrow_record();
                borrow_record_repo
                    .remove(&record.br_id)
                    .await
                    .map_err(ReturnBookError::RemoveBorrowRecordError)?;
            }

            Ok(())
        })
    })
    .await
    .map_err(|e| match e {
        UnitOfWorkError::Transaction(e) => e,
        UnitOfWorkError::Backend(e) => ReturnBookError::RepositoryError(FindBookError::Backend(e)),
    })
}

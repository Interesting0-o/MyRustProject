use crate::{
    AsyncUnitOfWork, BackendError, Book, FindBookError,
    adapter::book_repo::UpdateBookError as RepositoryUpdateBookError,
};

pub enum UpdateBookProfileError {
    NonExistBID,
    NoChanges,
    RepositoryError(BackendError),
}

pub async fn async_update_book<U: AsyncUnitOfWork>(
    uow: &mut U,
    bid: &str,
    name: Option<&str>,
    author: Option<&str>,
    price: Option<i64>,
    num: Option<u32>,
) -> Result<(), UpdateBookProfileError> {
    let mut repo_group = uow
        .repositories()
        .await
        .map_err(UpdateBookProfileError::RepositoryError)?;

    let book = match repo_group.book().find_book_by_bid(bid).await {
        Ok(book) => book,
        Err(FindBookError::NoResult) => return Err(UpdateBookProfileError::NonExistBID),
        Err(FindBookError::Backend(e)) => return Err(UpdateBookProfileError::RepositoryError(e)),
    };

    let updated_name = name
        .map(str::to_string)
        .unwrap_or_else(|| book.name.clone());
    let updated_author = author
        .map(str::to_string)
        .unwrap_or_else(|| book.author.clone());
    let updated_price = price.unwrap_or(book.price);
    let updated_num = num.unwrap_or(book.num);

    if updated_name == book.name
        && updated_author == book.author
        && updated_price == book.price
        && updated_num == book.num
    {
        return Err(UpdateBookProfileError::NoChanges);
    }

    repo_group
        .book()
        .update(Book {
            name: updated_name,
            author: updated_author,
            price: updated_price,
            num: updated_num,
            ..book
        })
        .await
        .map_err(|e| match e {
            RepositoryUpdateBookError::NonExistedBID => UpdateBookProfileError::NonExistBID,
            RepositoryUpdateBookError::Backend(e) => UpdateBookProfileError::RepositoryError(e),
        })
}

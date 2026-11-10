mod common;

use book_manager::{
    AsyncUnitOfWork, Book, ReturnBookError, adapter::borrow_record_repo::FindBorrowRecordError,
    async_return_book,
};

#[tokio::test]
async fn test_return_book() {
    let (pool, mut uow) = common::setup().await;
    let suffix = common::unique_suffix();
    let bid = format!("test_return_{suffix}");
    let account = format!("test_return_{suffix}");

    {
        let mut repos = uow.repositories().await.expect("获取仓库失败");
        repos
            .book()
            .add(Book {
                bid: bid.clone(),
                name: "还书测试".to_string(),
                author: "测试作者".to_string(),
                price: 1000,
                num: 3,
            })
            .await
            .expect("新增图书失败");
        repos
            .borrow_record()
            .add(&account, &bid)
            .await
            .expect("新增借阅记录失败");
    }

    let result = async_return_book(&mut uow, &account, &bid).await;
    assert!(matches!(result, Ok(())), "还书失败");

    let book = {
        let mut repos = uow.repositories().await.expect("获取仓库失败");
        repos
            .book()
            .find_book_by_bid(&bid)
            .await
            .expect("查询图书失败")
    };
    assert_eq!(book.num, 4, "还书后库存应加一");

    let records = {
        let mut repos = uow.repositories().await.expect("获取仓库失败");
        repos
            .borrow_record()
            .find_records_by_account_and_bid(&account, &bid)
            .await
    };
    assert!(
        matches!(records, Err(FindBorrowRecordError::NoResult)),
        "还书后借阅记录应被删除"
    );

    let result = async_return_book(&mut uow, &account, &bid).await;
    assert!(
        matches!(result, Err(ReturnBookError::NotFoundRecord)),
        "无借阅记录时应返回 NotFoundRecord"
    );

    let missing_bid = format!("test_missing_{suffix}");
    let result = async_return_book(&mut uow, &account, &missing_bid).await;
    assert!(
        matches!(result, Err(ReturnBookError::NonExistBID)),
        "不存在的图书应返回 NonExistBID"
    );

    common::delete_borrow_records(&pool, &account, &bid).await;
    common::delete_book(&pool, &bid).await;
}

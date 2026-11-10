mod common;

use book_manager::{AsyncUnitOfWork, Book, BorrowBookError, async_borrow_book};

#[tokio::test]
async fn test_borrow_book() {
    let (pool, mut uow) = common::setup().await;
    let suffix = common::unique_suffix();
    let bid = format!("test_borrow_{suffix}");
    let account = format!("test_borrow_{suffix}");

    {
        let mut repos = uow.repositories().await.expect("获取仓库失败");
        repos
            .book()
            .add(Book {
                bid: bid.clone(),
                name: "借书测试".to_string(),
                author: "测试作者".to_string(),
                price: 1000,
                num: 2,
            })
            .await
            .expect("新增图书失败");
    }

    let book = {
        let mut repos = uow.repositories().await.expect("获取仓库失败");
        repos
            .book()
            .find_book_by_bid(&bid)
            .await
            .expect("查询图书失败")
    };

    let result = async_borrow_book(&mut uow, &account, &bid).await;
    assert!(matches!(result, Ok(())), "借书失败");

    let book2 = {
        let mut repos = uow.repositories().await.expect("获取仓库失败");
        repos
            .book()
            .find_book_by_bid(&bid)
            .await
            .expect("查询图书失败")
    };

    let records = {
        let mut repos = uow.repositories().await.expect("获取仓库失败");
        repos
            .borrow_record()
            .find_records_by_account_and_bid(&account, &bid)
            .await
            .expect("查询借阅记录失败")
    };

    assert_eq!(book2.num, book.num - 1);
    assert!(!records.is_empty());

    let result = async_borrow_book(&mut uow, &account, &bid).await;
    assert!(matches!(result, Ok(())), "第二次借书失败");

    let result = async_borrow_book(&mut uow, &account, &bid).await;
    assert!(
        matches!(result, Err(BorrowBookError::BookNumIsZero)),
        "库存为零时应返回 BookNumIsZero"
    );

    let missing_bid = format!("test_missing_{suffix}");
    let result = async_borrow_book(&mut uow, &account, &missing_bid).await;
    assert!(
        matches!(result, Err(BorrowBookError::NonExistBID)),
        "不存在的图书应返回 NonExistBID"
    );

    common::delete_borrow_records(&pool, &account, &bid).await;
    common::delete_book(&pool, &bid).await;
}

mod common;

use book_manager::{AsyncUnitOfWork, Book, UpdateBookProfileError, async_update_book};

#[tokio::test]
async fn test_update_book() {
    let (pool, mut uow) = common::setup().await;
    let bid = format!("test_update_{}", common::unique_suffix());

    {
        let mut repos = uow.repositories().await.expect("获取仓库失败");
        repos
            .book()
            .add(Book {
                bid: bid.clone(),
                name: "旧书名".to_string(),
                author: "旧作者".to_string(),
                price: 1000,
                num: 5,
            })
            .await
            .expect("新增图书失败");
    }

    let result = async_update_book(
        &mut uow,
        &bid,
        Some("新书名"),
        Some("新作者"),
        Some(2000),
        Some(8),
    )
    .await;
    assert!(matches!(result, Ok(())), "更新图书失败");

    let book = {
        let mut repos = uow.repositories().await.expect("获取仓库失败");
        repos
            .book()
            .find_book_by_bid(&bid)
            .await
            .expect("查询图书失败")
    };
    assert_eq!(book.name, "新书名");
    assert_eq!(book.author, "新作者");
    assert_eq!(book.price, 2000);
    assert_eq!(book.num, 8);

    let result = async_update_book(&mut uow, &bid, Some("另一个书名"), None, None, None).await;
    assert!(matches!(result, Ok(())), "部分字段更新失败");

    let book = {
        let mut repos = uow.repositories().await.expect("获取仓库失败");
        repos
            .book()
            .find_book_by_bid(&bid)
            .await
            .expect("查询图书失败")
    };
    assert_eq!(book.name, "另一个书名");
    assert_eq!(book.author, "新作者");
    assert_eq!(book.price, 2000);
    assert_eq!(book.num, 8);

    let result = async_update_book(&mut uow, &bid, None, None, None, None).await;
    assert!(
        matches!(result, Err(UpdateBookProfileError::NoChanges)),
        "无改动时应返回 NoChanges"
    );

    let missing_bid = format!("test_missing_{}", common::unique_suffix());
    let result = async_update_book(&mut uow, &missing_bid, Some("书名"), None, None, None).await;
    assert!(
        matches!(result, Err(UpdateBookProfileError::NonExistBID)),
        "不存在的图书应返回 NonExistBID"
    );

    common::delete_book(&pool, &bid).await;
}

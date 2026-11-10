use book_manager::{AsyncUnitOfWork, MySqlUnitOfWork, async_borrow_book};
use sqlx::mysql::MySqlPoolOptions;

#[tokio::test]
async fn test_borrow_book() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL 未设置");
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("连接数据库失败");
    let mut uow = MySqlUnitOfWork::new(pool);

    let book = {
        let mut repos = uow.repositories().await.expect("获取仓库失败 1");
        let mut book_repo = repos.book();
        book_repo
            .find_book_by_bid("b001")
            .await
            .expect("查询失败 1")
    };

    async_borrow_book(&mut uow, "r001", "b001")
        .await
        .expect("借书失败");

    let book2 = {
        let mut repos = uow.repositories().await.expect("获取仓库失败 2");
        let mut book_repo = repos.book();
        book_repo
            .find_book_by_bid("b001")
            .await
            .expect("查询失败 2")
    };

    let records = {
        let mut repos = uow.repositories().await.expect("获取仓库失败 3");
        let mut borrow_record_repo = repos.borrow_record();
        borrow_record_repo
            .find_records_by_account_and_bid("r001", "b001")
            .await
            .expect("查询借阅记录失败")
    };

    assert_eq!(book2.num, book.num - 1);
    assert!(!records.is_empty());
}

use book_manager::{MySqlUnitOfWork, service::login::async_reader_login};

use sqlx::mysql::MySqlPoolOptions;

#[tokio::test]
async fn test_reader_login() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL 未设置");

    let account = "r001";
    let pwd = "123456";
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("连接数据库失败");

    let mut uow = MySqlUnitOfWork::new(pool);
    async_reader_login(&mut uow, account, pwd)
        .await
        .expect("登录函数异常");
}

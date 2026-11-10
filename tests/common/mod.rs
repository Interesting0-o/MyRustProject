#![allow(dead_code)]

use book_manager::MySqlUnitOfWork;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};

pub async fn connect() -> MySqlPool {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL 未设置");
    MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("连接数据库失败")
}

pub async fn setup() -> (MySqlPool, MySqlUnitOfWork) {
    let pool = connect().await;
    (pool.clone(), MySqlUnitOfWork::new(pool))
}

pub fn unique_suffix() -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("系统时间早于 UNIX_EPOCH")
        .as_nanos();
    format!("{}_{}", std::process::id(), nanos)
}

pub async fn delete_book(pool: &MySqlPool, bid: &str) {
    sqlx::query("DELETE FROM books WHERE bid = ?")
        .bind(bid)
        .execute(pool)
        .await
        .expect("清理图书失败");
}

pub async fn delete_reader(pool: &MySqlPool, account: &str) {
    sqlx::query("DELETE FROM readers WHERE account = ?")
        .bind(account)
        .execute(pool)
        .await
        .expect("清理读者失败");
}

pub async fn delete_librarian(pool: &MySqlPool, account: &str) {
    sqlx::query("DELETE FROM librarian WHERE account = ?")
        .bind(account)
        .execute(pool)
        .await
        .expect("清理管理员失败");
}

pub async fn delete_borrow_records(pool: &MySqlPool, account: &str, bid: &str) {
    sqlx::query("DELETE FROM borrow_record WHERE account = ? AND bid = ?")
        .bind(account)
        .bind(bid)
        .execute(pool)
        .await
        .expect("清理借阅记录失败");
}

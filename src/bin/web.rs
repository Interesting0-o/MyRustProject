use axum::Router;
use book_manager::api::{
    AppState, borrow_book, find_book, login, return_book, sign_up, update_book, update_reader,
};
use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL 未设置");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET 未设置");
    let jwt_expires_secs = match std::env::var("JWT_EXPIRES_SECS") {
        Ok(res) => res.parse::<u64>().expect("JWT_EXPIRES_SECS 字段设置无效"),
        Err(_) => (2 * 60 * 60) as u64,
    };
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("连接数据库失败");

    let app = Router::new()
        .merge(login::router())
        .merge(sign_up::router())
        .merge(borrow_book::router())
        .merge(return_book::router())
        .merge(find_book::router())
        .merge(update_book::router())
        .merge(update_reader::router())
        .with_state(AppState {
            pool,
            jwt_secret,
            jwt_expires_secs,
        });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

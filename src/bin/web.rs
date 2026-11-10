use axum::{Router, middleware};
use book_manager::api::{
    AppState, auth::auth_middleware, borrow_book, find_book, login, return_book, sign_up,
    update_book, update_reader,
};
use book_manager::{
    new_book_table, new_borrow_record_table, new_librarian_table, new_reader_table,
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

    new_reader_table(&pool).await.expect("创建 readers 表失败");
    new_librarian_table(&pool)
        .await
        .expect("创建 librarian 表失败");
    new_book_table(&pool).await.expect("创建 books 表失败");
    new_borrow_record_table(&pool)
        .await
        .expect("创建 borrow_record 表失败");

    let state = AppState {
        pool,
        jwt_secret,
        jwt_expires_secs,
    };
    let app_protect = Router::new()
        .merge(borrow_book::router())
        .merge(update_book::router())
        .merge(find_book::router())
        .merge(update_reader::router())
        .merge(return_book::router())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    let app = Router::new()
        .merge(login::router())
        .merge(sign_up::router())
        .merge(app_protect)
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

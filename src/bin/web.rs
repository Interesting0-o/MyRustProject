use axum::{Router, middleware};
use book_manager::api::hello;
use book_manager::api::{
    AppState, auth::auth_middleware, borrow_book, find_book, logging::request_log_middleware,
    login, return_book, sign_up, update_book, update_reader,
};
use book_manager::run_migrations;
use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL 未设置");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET 未设置");
    let backend_url = std::env::var("BACKEND_URL").expect("BACKEND_URL 未设置");
    let backend_port = std::env::var("BACKEND_PORT").expect("BACKEND_PORT未设置");
    let jwt_expires_secs = match std::env::var("JWT_EXPIRES_SECS") {
        Ok(res) => res.parse::<u64>().expect("JWT_EXPIRES_SECS 字段设置无效"),
        Err(_) => (2 * 60 * 60) as u64,
    };
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("连接数据库失败");

    run_migrations(&pool).await.expect("执行数据库迁移失败");

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
        .merge(hello::router())
        .merge(app_protect)
        .layer(middleware::from_fn(request_log_middleware))
        .with_state(state.clone());

    let url = format!("{}:{}", backend_url, backend_port);
    let listener = tokio::net::TcpListener::bind(&url).await.unwrap();
    println!("Serve on {}", url);

    axum::serve(listener, app).await.unwrap();
}

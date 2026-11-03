use axum::Router;
use book_manager::api::{
    AppState, borrow_book, find_book, login, return_book, sign_up, update_book, update_reader,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(login::router())
        .merge(sign_up::router())
        .merge(borrow_book::router())
        .merge(return_book::router())
        .merge(find_book::router())
        .merge(update_book::router())
        .merge(update_reader::router())
        .with_state(AppState {
            db_path: "resource/repo.db".to_string(),
        });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

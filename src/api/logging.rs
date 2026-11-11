use std::time::Instant;

use axum::{extract::Request, middleware::Next, response::Response};

pub async fn request_log_middleware(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let started = Instant::now();

    let response = next.run(request).await;

    println!(
        "[API] {} {} -> {} ({} ms)",
        method,
        uri,
        response.status().as_u16(),
        started.elapsed().as_millis()
    );

    response
}

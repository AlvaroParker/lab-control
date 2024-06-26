use axum::{http::Request, middleware::Next, response::Response};

// Logs layer, every request will be logged
pub async fn log_layer(request: Request<axum::body::Body>, next: Next) -> Result<Response, ()> {
    tracing::info!("{} Request to {}", request.method(), request.uri());
    tracing::info!("{:#?}", request.headers());
    Ok(next.run(request).await)
}

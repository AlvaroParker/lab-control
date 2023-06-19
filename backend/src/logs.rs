use axum::{http::Request, middleware::Next, response::Response};

pub async fn log_layer<B>(request: Request<B>, next: Next<B>) -> Result<Response, ()> {
    tracing::info!("{} Request to {}", request.method(), request.uri());
    tracing::info!("{:#?}", request.headers());
    //request.headers()

    return Ok(next.run(request).await);
}

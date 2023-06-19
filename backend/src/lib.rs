pub mod api;
pub mod database;
mod logs;

use std::sync::Arc;

use axum::middleware;
use logs::log_layer;

use crate::api::routes::create_routes;
use crate::database::pool::create_pool;

pub async fn run() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let pool = create_pool().await;
    let pool_state = Arc::new(pool);

    let app = create_routes(pool_state.clone())
        .await
        .route_layer(middleware::from_fn(log_layer));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

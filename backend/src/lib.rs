use axum_server::Server;
use lazy_static::lazy_static;

// We declare static env variables or use the defaults
lazy_static! {
    pub static ref DATABASE_URL: String =
        std::env::var("DATABASE_URL").expect("Failed to get DATABASE_URL env var");
    pub static ref SOCKET_IP: String =
        std::env::var("SOCKET_IP").expect("Failed to get SOCKET_IP env var");
    pub static ref SOCKET_PORT: String =
        std::env::var("SOCKET_PORT").expect("Failed to get SOCKET_PORT env var");
}
pub mod api;
pub mod database;
mod logs;

use std::sync::Arc;

use axum::middleware;
use logs::log_layer;

use crate::api::routes::create_routes;
use crate::database::pool::create_pool;
// Run the webserver
pub async fn run() {
    // We initialize the tracing subscriber, this allows us to log
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    #[cfg(not(debug_assertions))]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Print log
    tracing::info!(
        "Listening for print on {}:{}",
        SOCKET_IP.as_str(),
        SOCKET_PORT.as_str()
    );
    // We create the DB pool, Arc allows us to share the pool between threads safely
    let pool = create_pool().await;
    let pool_state = Arc::new(pool);
    let app = create_routes(pool_state.clone())
        .await
        .route_layer(middleware::from_fn(log_layer));

    // We bind the server to the host ip and to the port 3000
    Server::bind("0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

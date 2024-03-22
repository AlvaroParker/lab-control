use axum_server::Server;
use lazy_static::lazy_static;

// We declare static env variables or use the defaults
lazy_static! {
    pub static ref DATABASE_URL: String = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:defensa2001@database/fingerprints".to_string());
    pub static ref SOCKET_IP: String =
        std::env::var("SOCKET_IP").unwrap_or("192.168.68.112".into());
    pub static ref SOCKET_PORT: String = std::env::var("SOCKET_PORT").unwrap_or("5000".into());
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
    // We get the socket ip and port from the static variable defined above
    let sock_ip = std::env::var("SOCKET_IP").unwrap_or("127.0.0.1".to_string());
    let sock_port = std::env::var("SOCKET_PORT").unwrap_or("5000".to_string());
    // Print log
    println!("Listening for print on {}:{}", sock_ip, sock_port);
    // We initialize the tracing subscriber, this allows us to log
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
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

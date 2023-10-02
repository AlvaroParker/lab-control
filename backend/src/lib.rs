use axum::http::Method;
use lazy_static::lazy_static;
use tower_http::cors::{Any, CorsLayer};
use utoipa_swagger_ui::SwaggerUi;
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
use tower_http;
use utoipa::openapi;

use crate::api::routes::create_routes;
use crate::database::pool::create_pool;
// Run the webserver
pub async fn run(docs: Option<openapi::OpenApi>) {
    // We get the socket ip and port from the static variable defined above
    let sock_ip = std::env::var("SOCKET_IP").unwrap_or("127.0.0.1".to_string());
    let sock_port = std::env::var("SOCKET_PORT").unwrap_or("5000".to_string());
    // Print log
    println!("Listening for print on {}:{}", sock_ip, sock_port);
    // We initialize the tracing subscriber, this allows us to log
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cors = CorsLayer::new()
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::OPTIONS,
        ])
        .allow_origin(Any)
        .allow_credentials(false);

    // We create the DB pool, Arc allows us to share the pool between threads safely
    let pool = create_pool().await;
    let pool_state = Arc::new(pool);
    let mut app = create_routes(pool_state.clone())
        .await
        .route_layer(middleware::from_fn(log_layer))
        .layer(cors);

    app = if let Some(docs) = docs {
        app.merge(SwaggerUi::new("/api/docs").url("/api/docs/openapi.json", docs))
    } else {
        app
    };

    // We bind the server to the host ip and to the port 3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

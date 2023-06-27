use super::{admins, personas, registros};
use crate::database::pool::Pool;
use axum::{http::Method, Router};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub async fn create_routes(state: Arc<Pool>) -> Router {
    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_origin(Any)
        .allow_methods([
            Method::POST,
            Method::GET,
            Method::DELETE,
            Method::OPTIONS,
            Method::PUT,
        ]);
    Router::new()
        .nest(
            "/api/usuarios",
            personas::routes::create_routes(state.clone()).await,
        )
        .nest(
            "/api/admin",
            admins::routes::_create_routes(state.clone()).await,
        )
        .nest(
            "/api/registros",
            registros::routes::create_routes(state.clone()).await,
        )
        .with_state(state)
        .layer(cors)
}

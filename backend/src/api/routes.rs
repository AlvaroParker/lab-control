use super::{admins, motivos, personas, registros};
use crate::database::pool::Pool;
use axum::{http::Method, Router};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

// Create the routes of our webserver, state is the DB Pool, where
// we can get a connection to the DB
pub async fn create_routes(state: Arc<Pool>) -> Router {
    // We create a CORS layer, this allows us to make requests from
    // different origins
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
    // Create the router with layer CORS and state DB Pool
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
        .nest(
            "/api/motivos",
            motivos::routes::create_routes(state.clone()).await,
        )
        .with_state(state)
        .layer(cors)
}

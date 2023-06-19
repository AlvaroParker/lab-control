use crate::{api::guard::guard_layer, database::pool::Pool};

// use axum::routing::{delete, get, post};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use super::controller::{get_all, registrar_rut};

// Create the routes with the provided handlers
pub async fn create_routes() -> Router<Arc<Pool>> {
    Router::new()
        .route("/", get(get_all))
        .route_layer(middleware::from_fn(guard_layer))
        .route("/", post(registrar_rut))
}

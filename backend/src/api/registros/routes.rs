use crate::{api::guard::guard_layer, database::pool::Pool};

// use axum::routing::{delete, get, post};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use super::controller::{get_all, get_last, get_last_month, registrar_rut};

// Create the routes with the provided handlers
pub async fn create_routes(pool: Arc<Pool>) -> Router<Arc<Pool>> {
    Router::new()
        .with_state(pool.clone())
        .route("/", get(get_all))
        .route("/reporte", get(get_last_month))
        .route("/last/:rut", get(get_last))
        .route_layer(middleware::from_fn_with_state(pool.clone(), guard_layer))
        .route("/", post(registrar_rut))
}

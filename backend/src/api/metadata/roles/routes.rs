// use axum::routing::{delete, get, post};
use axum::{
    routing::{delete, get, post},
    Router,
};
use std::sync::Arc;

use crate::database::pool::Pool;

use super::controller::{delete_rol, get_all, post_rol};

// Create the routes with the provided handlers
pub async fn create_routes(pool: Arc<Pool>) -> Router<Arc<Pool>> {
    Router::new()
        .with_state(pool.clone())
        .route("/", get(get_all))
        .route("/", post(post_rol))
        .route("/:id", delete(delete_rol))
}

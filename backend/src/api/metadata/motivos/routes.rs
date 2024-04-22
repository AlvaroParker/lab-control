// use axum::routing::{delete, get, post};
use axum::{
    routing::{delete, get, post},
    Router,
};
use std::sync::Arc;

use crate::database::pool::Pool;

use super::controller::{delete_motivo, get_all, post_motivo};

// Create the routes with the provided handlers
pub async fn create_routes(pool: Arc<Pool>) -> Router<Arc<Pool>> {
    Router::new()
        .with_state(pool.clone())
        .route("/", get(get_all))
        .route("/", post(post_motivo))
        .route("/:id", delete(delete_motivo))
}

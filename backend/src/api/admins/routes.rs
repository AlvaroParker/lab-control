use crate::{api::guard::guard_layer, database::pool::Pool};

// use axum::routing::{delete, get, post};
use axum::{middleware, routing::get, routing::post, Router};
use std::sync::Arc;

use super::controller::{auth, create_user, login};

pub async fn _create_routes() -> Router<Arc<Pool>> {
    Router::new()
        .route("/auth", get(auth))
        .route_layer(middleware::from_fn(guard_layer))
        .route("/login", post(login))
        .route("/signin", post(create_user))
}

use crate::{api::guard::guard_layer, database::pool::Pool};
use axum::{middleware, routing::get, routing::post, Router};
use std::sync::Arc;

// We get the functions from the controller
use super::controller::{auth, create_user, login, logout};

pub async fn _create_routes(pool: Arc<Pool>) -> Router<Arc<Pool>> {
    // Create a new route for each relevant function in the controller
    Router::new()
        .route("/auth", get(auth))
        .route("/signin", post(create_user))
        .route_layer(middleware::from_fn_with_state(pool, guard_layer))
        .route("/login", post(login))
        .route("/logout", post(logout))
}

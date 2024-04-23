use std::sync::Arc;

use axum::{middleware, Router};

use super::guard::guard_layer;
use crate::database::pool::Pool;

mod motivos;
mod roles;

pub async fn create_routes(pool: Arc<Pool>) -> Router<Arc<Pool>> {
    Router::new()
        .nest("/roles", roles::routes::create_routes(pool.clone()).await)
        .route_layer(middleware::from_fn_with_state(pool.clone(), guard_layer))
        .nest(
            "/motivos",
            motivos::routes::create_routes(pool.clone()).await,
        )
}

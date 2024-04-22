use std::sync::Arc;

use axum::Router;

use crate::database::pool::Pool;

mod motivos;
mod roles;

pub async fn create_routes(pool: Arc<Pool>) -> Router<Arc<Pool>> {
    Router::new()
        .nest(
            "/motivos",
            motivos::routes::create_routes(pool.clone()).await,
        )
        .nest("/roles", roles::routes::create_routes(pool.clone()).await)
}

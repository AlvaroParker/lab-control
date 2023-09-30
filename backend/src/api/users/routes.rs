use super::controller::{
    edit_user_by_rut, enroll_user, get_all_users, get_user_by_rut, remove_user_by_rut, verify_user,
};
use crate::{api::guard::guard_layer, database::pool::Pool};

use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

// Create the routes
pub async fn create_routes(pool: Arc<Pool>) -> Router<Arc<Pool>> {
    Router::new()
        .route("/", get(get_all_users)) //Get all alumnos
        .route("/enroll", get(enroll_user)) // Create a new alumno
        .route("/:rut", put(edit_user_by_rut))
        .route("/:rut", get(get_user_by_rut)) // Get an alumno by rut
        .route("/:rut", delete(remove_user_by_rut)) // Delete an alumnos by rut
        .route_layer(middleware::from_fn_with_state(pool, guard_layer))
        // Verify shouldn't need authorization
        .route("/verify", post(verify_user)) // Verify alumno
}

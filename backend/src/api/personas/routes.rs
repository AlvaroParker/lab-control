use super::controller::{
    edit_persona_by_rut, enroll_persona, get_all_personas, get_persona_by_rut,
    manual_verify_persona, remove_persona_by_rut, reroll_persona, search_persona, verify_persona,
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
        .route("/", get(get_all_personas)) //Get all alumnos
        .route("/enroll", get(enroll_persona)) // Create a new alumno
        .route("/reroll", get(reroll_persona)) // Create a new alumno
        .route("/:rut", put(edit_persona_by_rut))
        .route("/:rut", get(get_persona_by_rut)) // Get an alumno by rut
        .route("/:rut", delete(remove_persona_by_rut)) // Delete an alumnos by rut
        .route_layer(middleware::from_fn_with_state(pool, guard_layer))
        .route("/search", get(search_persona)) // Search persona
        // Verify shouldn't need auth
        .route("/verify/manual", post(manual_verify_persona)) // Manually verify alumno by rut
        .route("/verify", post(verify_persona)) // Verify alumno
}

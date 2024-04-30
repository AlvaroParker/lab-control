use crate::{api::guard::guard_layer, database::pool::Pool};
use axum::{middleware, routing::get, routing::post, Router};
use std::sync::Arc;

// We get the functions from the controller
use super::controller::{
    auth, change_password, create_user, delete_admin, get_admins, login, logout,
};

pub async fn _create_routes(pool: Arc<Pool>) -> Router<Arc<Pool>> {
    // Create a new route for each relevant function in the controller
    Router::new()
        // Return status: [OK 200, UNAUTHORIZED 401, INTERNAL SERVER ERROR 500]
        .route("/auth", get(auth))
        // Return status: [OK 200, UNAUTHORIZED 401, CONFLICT 409, INTERNAL SERVER ERROR 500]
        .route("/signin", post(create_user))
        // Return status: [OK 200, UNAUTHORIZED 401, INTERNAL SERVER ERROR 500]
        .route("/all", get(get_admins))
        // Return status: [OK 200, UNAUTHORIZED 401, INTERNAL SERVER ERROR 500]
        .route("/change", post(change_password))
        // Return status: [OK 200, UNAUTHORIZED 401, NOT FOUND 404, INTERNAL SERVER ERROR 500]
        .route("/delete/:id", post(delete_admin))
        .route_layer(middleware::from_fn_with_state(pool, guard_layer))
        // Return status: [OK 200, UNAUTHORIZED 401, INTERNAL SERVER ERROR 500]
        .route("/login", post(login))
        // Return status: [OK 200, UNAUTHORIZED 401, INTERNAL SERVER ERROR 500]
        .route("/logout", post(logout))
}

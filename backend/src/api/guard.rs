use std::sync::Arc;

use async_session::SessionStore;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;

use crate::database::pool::Pool;

use super::utils::handle_cookie_err;

// Guard middleware
// This function is called before almost every route
pub async fn guard_layer<B>(
    State(pool): State<Arc<Pool>>,
    jar: CookieJar,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, (StatusCode, String)> {
    // We check if the cookie jar has an `auth-cookie` set,
    // if it does, we check if the cookie is valid
    if let Some(session_id) = jar.get("auth-cookie") {
        // We get the cookie value
        let cookie_value = session_id.value();
        // We check if the cookie is a valid session by querying the DB
        // for a session with the same cookie value
        let cookie = pool
            .get_store()
            .load_session(cookie_value.into())
            .await
            .map_err(handle_cookie_err)?;
        match cookie {
            // If the cookie exists, we call the next middleware/route
            Some(_cookie) => Ok(next.run(request).await),
            // If the cookie doesn't exists, return 401
            None => Err((StatusCode::UNAUTHORIZED, "Unauthorized".into())),
        }
    } else {
        // If the cookie doesn't exists, return 401
        Err((StatusCode::UNAUTHORIZED, "Unauthorized".into()))
    }
}

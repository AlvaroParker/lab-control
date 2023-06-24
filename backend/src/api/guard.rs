use std::sync::Arc;

use async_session::SessionStore;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::database::pool::Pool;

use super::utils::{handle_cookie_err, internal_error};

// Guard middleware
// Todo: Investigate more into JSWT and implement refresh tokens
pub async fn guard_layer<B>(
    State(pool): State<Arc<Pool>>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, (StatusCode, String)> {
    let cookie = request
        .headers()
        .get("cookie-auth")
        .ok_or((StatusCode::BAD_REQUEST, "Missing auth cookie".into()))?
        .to_str()
        .map_err(internal_error)?;

    let cookie = pool
        .get_store()
        .load_session(cookie.into())
        .await
        .map_err(handle_cookie_err)?; // Err should mean Internal server error (500)

    match cookie {
        // If the session doesn't exist, this will be None
        Some(_cookie) => Ok(next.run(request).await),
        None => Err((StatusCode::UNAUTHORIZED, "Unauthorized".into())),
    }
}

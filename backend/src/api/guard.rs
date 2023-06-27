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
// Todo: Investigate more into JSWT and implement refresh tokens
pub async fn guard_layer<B>(
    State(pool): State<Arc<Pool>>,
    jar: CookieJar,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, (StatusCode, String)> {
    if let Some(session_id) = jar.get("auth-cookie") {
        let cookie_value = session_id.value();
        let cookie = pool
            .get_store()
            .load_session(cookie_value.into())
            .await
            .map_err(handle_cookie_err)?;
        match cookie {
            Some(_cookie) => Ok(next.run(request).await),
            None => Err((StatusCode::UNAUTHORIZED, "Unauthorized".into())),
        }
    } else {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".into()));
    }
}

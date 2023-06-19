use axum::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use super::utils::is_valid;

// Guard middleware
// Todo: Investigate more into JSWT and implement refresh tokens
pub async fn guard_layer<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, (StatusCode, String)> {
    let token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or((
            StatusCode::BAD_REQUEST,
            "Bad request: missing authorization header".into(),
        ))?
        .token()
        .to_owned();
    is_valid(&token)?;
    return Ok(next.run(request).await);
}

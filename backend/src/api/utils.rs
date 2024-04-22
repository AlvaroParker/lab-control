use axum::http::StatusCode;
use regex;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Salida {
    pub salida: bool,
}
pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    dbg!(&err);
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
// This might be to much overhead for the server
pub fn is_valid_num_rut(rut: &str) -> bool {
    let follow_rut = regex::Regex::new(r"^\d{8}[0-9kK]$").unwrap();
    let follow_rut_7 = regex::Regex::new(r"^\d{7}[0-9kK]$").unwrap();
    (rut.len() == 9 && follow_rut.is_match(rut)) || (rut.len() == 8 && follow_rut_7.is_match(rut))
}

// Handle cookie errors, return 500 internal server error
pub fn handle_cookie_err(err: async_session::Error) -> (StatusCode, String) {
    let err_msg = err.to_string();
    tracing::info!(err_msg);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal Server Error".into(),
    )
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    exp: usize,
    iat: usize,
}

pub fn conflict_err(err: DbErr, message: &str) -> (StatusCode, String) {
    if let DbErr::Query(sea_orm::RuntimeErr::SqlxError(sqlx::Error::Database(e))) = err {
        // Try to cast Box<dyn Error> to PgDatabaseError
        if let Ok(err) = e.try_downcast::<sqlx::postgres::PgDatabaseError>() {
            match err.code() {
                "23505" => return (StatusCode::BAD_REQUEST, message.to_string()),
                "23502" => {
                    return (
                        StatusCode::BAD_REQUEST,
                        "Missing required field".to_string(),
                    )
                }
                _ => {}
            }
        }
    }
    // Return internal error if the error is not a duplicate key error
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal Server Error".to_string(),
    )
}

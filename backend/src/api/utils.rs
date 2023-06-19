use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{self, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use regex;
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
pub fn is_valid_num_rut(rut: &String) -> bool {
    let follow_rut = regex::Regex::new(r"^\d{8}[0-9kK]$").unwrap();
    let follow_rut_7 = regex::Regex::new(r"^\d{7}[0-9kK]$").unwrap();
    (rut.len() == 9 && follow_rut.is_match(rut)) || (rut.len() == 8 && follow_rut_7.is_match(rut))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    exp: usize,
    iat: usize,
}

pub fn create_jwt() -> Result<String, (StatusCode, String)> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::hours(4);
    let exp = (now + expires_in).timestamp() as usize;

    let claim = Claims { exp, iat };

    let secret = EncodingKey::from_secret(dotenv!("JWT_SECRET").as_bytes());

    jsonwebtoken::encode(&Header::default(), &claim, &secret).map_err(internal_error)
}

pub fn is_valid(token: &str) -> Result<(), (StatusCode, String)> {
    let secret = DecodingKey::from_secret(dotenv!("JWT_SECRET").as_bytes());
    let _result =
        jsonwebtoken::decode::<Claims>(token, &secret, &Validation::new(Algorithm::HS256))
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()))?;
    Ok(())
}

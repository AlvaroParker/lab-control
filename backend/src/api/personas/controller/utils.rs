use crate::api::utils::internal_error;
use crate::database::entities::personas;
use crate::{database::entities::personas::Entity as Personas, database::pool::Pool};
use axum::http::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// NewPersona struct used to on `enroll.rs` to enroll a new Persona
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NewPersona {
    pub nombre: String,
    pub apellido_1: String,
    pub apellido_2: String,
    pub rut: String,
    pub correo_uai: String,
    pub is_disabled: bool,
    pub rol: String,
}

// We added the print_path so we a can then deserialize Outer and Serialize it again to an `personas::Model`
#[derive(Debug, Serialize)]
pub struct Outer<'a> {
    #[serde(flatten)]
    pub field_1: NewPersona,
    pub print_path: &'a str,
}

// Create a new Outer instance with a `NewPersona` struct and a `print_path` String
impl<'a> Outer<'a> {
    pub fn new(field_1: NewPersona, print_path: &'a str) -> Self {
        Self {
            field_1,
            print_path,
        }
    }
}
// This struct will be Deserialized into JSON and passed in the body of a socket request
// to the fingerprint sensor. It carries the fingerprint file path to check for a match.
#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
    pub action: Action,
    pub paths: Vec<String>,
}

// Action, either `verify` to Verify a print or `Enroll` to enroll a new print
#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    #[serde(alias = "verify")]
    Verify,
    #[serde(alias = "enroll")]
    Enroll,
}

// Check if the rust exists in the database, this usually isn't used as we handle the DB unique value contraint error instead of doing
// 2 queries.
pub async fn rut_exists_or_email(
    pool: Arc<Pool>,
    rut: &String,
    email: &String,
) -> Result<bool, (StatusCode, String)> {
    let querie = Personas::find().filter(
        personas::Column::Rut
            .eq(rut)
            .or(personas::Column::CorreoUai.eq(email)),
    );
    Ok(querie
        .one(pool.get_db())
        .await
        .map_err(internal_error)?
        .is_some())
}

use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::Query;
use axum::{extract::State, http::StatusCode, Json};
use chrono::{Local, Utc};
use sea_orm::prelude::DateTimeWithTimeZone;

use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, DbBackend, FromQueryResult, Statement};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::utils::internal_error;
use crate::database::entities::registros;
use crate::database::pool::Pool;

// The json that we will respont with will have the following data:
#[derive(Serialize, Deserialize, Debug)]
pub struct RegistroAlumno {
    pub id: i32,
    pub nombre: String,
    pub apellido_1: String,
    pub apellido_2: String,
    pub rut: String,
    pub correo_uai: String,
    pub fecha: DateTimeWithTimeZone,
    pub salida: bool,
    pub rol: String,
    pub motivo: String,
}

// Get all the registros
pub async fn get_all(
    State(pool): State<Arc<Pool>>,
    Query(params): Query<HashMap<String, i32>>,
) -> Result<Json<Vec<RegistroAlumno>>, (StatusCode, String)> {
    // Use the limit and offset provided, if none use default
    let limit = params.get("limit".into()).unwrap_or(&100);
    let offset = params.get("offset".into()).unwrap_or(&0);
    // Run custom querie to join `registros` table and `personas` table
    let querie = format!(
        r#"SELECT registros.id, registros.rut, registros.fecha, registros.salida, registros.motivo, personas.nombre, personas.apellido_1, personas.apellido_2, personas.correo_uai, personas.rol FROM registros JOIN personas ON registros.rut = personas.rut ORDER BY registros.fecha DESC LIMIT {} OFFSET {};"#,
        limit, offset
    );
    // Querie the database and store the result in a Vector of type `Value` (`serde_json`)
    let unique: Vec<Value> = sea_orm::query::JsonValue::find_by_statement(
        Statement::from_sql_and_values(DbBackend::Postgres, &querie, []),
    )
    .all(pool.get_db())
    .await
    .map_err(internal_error)?;

    // Serialize those values into structs of type `RegistroAlumno`
    let registros_rows: Vec<RegistroAlumno> =
        serde_json::from_value(serde_json::Value::Array(unique)).map_err(internal_error)?;

    // Return the registros
    Ok(Json(registros_rows))
}

#[derive(Serialize, Deserialize)]
pub struct RegistroNew {
    pub rut: String,
    pub salida: bool,
    pub motivo: String,
}

pub async fn registrar_rut(
    State(pool): State<Arc<Pool>>,
    Json(registro): Json<RegistroNew>,
) -> Result<(), (StatusCode, String)> {
    let mut querie = registros::ActiveModel::new();

    let now = Local::now();
    let offset_in_sec = now.offset();
    let now = Utc::now().with_timezone(offset_in_sec);

    querie.rut = Set(registro.rut);
    querie.fecha = Set(now);
    querie.salida = Set(registro.salida);
    querie.id = NotSet;
    querie.motivo = Set(registro.motivo);
    querie.insert(pool.get_db()).await.map_err(internal_error)?;
    Ok(())
}

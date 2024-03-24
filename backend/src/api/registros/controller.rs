use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::{Path, Query};
use axum::http::header;
use axum::response::IntoResponse;
use axum::{extract::State, http::StatusCode, Json};
use chrono::{Datelike, Local, Utc};
use sea_orm::prelude::DateTimeWithTimeZone;

use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, DbBackend, EntityTrait, FromQueryResult,
    QueryFilter, QueryOrder, Statement,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::utils::{internal_error, is_valid_num_rut};
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
    let limit = params.get("limit").unwrap_or(&100);
    let offset = params.get("offset").unwrap_or(&0);
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

pub async fn get_last_month(
    State(pool): State<Arc<Pool>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let now = Utc::now();
    let month = format!("{}-{:02}-{:02}", now.year(), now.month(), 1);
    let query = format!(
        r#"SELECT registros.id, registros.rut, registros.fecha, registros.salida, 
    registros.motivo, personas.nombre, personas.apellido_1, personas.apellido_2, 
    personas.correo_uai, personas.rol FROM registros 
    JOIN personas ON registros.rut = personas.rut 
    WHERE registros.fecha >= '{}' ORDER BY registros.fecha DESC;"#,
        month
    );

    let values: Vec<Value> = sea_orm::query::JsonValue::find_by_statement(Statement::from_string(
        DbBackend::Postgres,
        &query,
    ))
    .all(pool.get_db())
    .await
    .map_err(internal_error)?;

    let registros_rows: Vec<RegistroAlumno> =
        serde_json::from_value(serde_json::Value::Array(values)).map_err(internal_error)?;

    let buff = Vec::new();
    let mut wrtr = csv::Writer::from_writer(buff);

    for registro in registros_rows {
        wrtr.serialize(registro).map_err(internal_error)?;
    }

    let headers = [
        (header::CONTENT_TYPE, "text/csv; charset=utf-8"),
        (
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"Reporte.csv\"",
        ),
    ];
    let v = wrtr.into_inner().map_err(internal_error)?;
    Ok((headers, v))
}

// RegistroNew is the json body that we will receive when registering a new registro
#[derive(Serialize, Deserialize)]
pub struct RegistroNew {
    pub rut: String,
    pub salida: bool,
    pub motivo: String,
}

// Register a new reigstro into the DB
pub async fn registrar_rut(
    State(pool): State<Arc<Pool>>,
    Json(registro): Json<RegistroNew>,
) -> Result<(), (StatusCode, String)> {
    // Create a new registro active model
    let mut querie = registros::ActiveModel::new();

    // Get current date
    let now = Local::now();
    let offset_in_sec = now.offset();
    let now = Utc::now().with_timezone(offset_in_sec);

    // Build the new registro
    querie.rut = Set(registro.rut);
    querie.fecha = Set(now);
    querie.salida = Set(registro.salida);
    querie.id = NotSet;
    querie.motivo = Set(registro.motivo);
    // Insert the new registro into the DB
    querie.insert(pool.get_db()).await.map_err(internal_error)?;
    Ok(())
}

// Get last registro of a rut
pub async fn get_last(
    State(pool): State<Arc<Pool>>,
    Path(rut): Path<String>,
) -> Result<Json<registros::Model>, (StatusCode, String)> {
    if is_valid_num_rut(&rut) {
        let query = registros::Entity::find()
            .filter(registros::Column::Rut.eq(rut))
            .order_by_desc(registros::Column::Fecha)
            .one(pool.get_db())
            .await
            .map_err(internal_error)?;
        if let Some(query) = query {
            return Ok(Json(query));
        } else {
            return Err((StatusCode::NO_CONTENT, "".into()));
        }
    }
    Err((StatusCode::BAD_REQUEST, "Rut invalido".into()))
}

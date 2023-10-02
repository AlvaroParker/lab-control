use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::{Path, Query};
use axum::{extract::State, http::StatusCode, Json};
use chrono::{Local, Utc};
use sea_orm::prelude::DateTimeWithTimeZone;

use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, DbBackend, EntityTrait, FromQueryResult,
    QueryFilter, QueryOrder, Statement,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

use crate::api::utils::{internal_error, is_valid_num_rut};
use crate::database::entities::registros;
use crate::database::pool::Pool;

// The json that we will respont with will have the following data:
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct RegistroAlumno {
    pub id: i32,
    pub nombre: String,
    pub apellido_1: String,
    pub apellido_2: String,
    pub rut: String,
    pub correo_uai: String,
    #[schema(value_type = String, format = "date-time")]
    pub fecha: DateTimeWithTimeZone,
    pub salida: bool,
    pub rol: String,
    pub motivo: String,
}

/// Get all the registros
#[utoipa::path(
   get,
    path = "/registros",
    tag = "Registros",
    responses(
        (status = 200, description = "Success", body = [RegistroAlumno], examples((
            "Demo" = (description = "Long description",
                value = json!([{
                    "id": 1,
                    "nombre": "Bruce",
                    "apellido_1": "Banner",
                    "apellido_2": "The Hulk",
                    "rut": "12345678-9",
                    "correo_uai": "bbanner@alumnos.uai.cl",
                    "fecha": "2021-05-31T15:00:00-03:00",
                    "salida": true,
                    "rol": "Alumno",
                    "motivo": "salida"
                }])
            )),
        )),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    request_body = RequestAdmin,
    security(
        ("auth-cookie" = [])
    )
)]
pub async fn get_all(
    State(pool): State<Arc<Pool>>,
    Query(params): Query<HashMap<String, i32>>,
) -> Result<Json<Vec<RegistroAlumno>>, (StatusCode, String)> {
    // Use the limit and offset provided, if none use default
    let limit = params.get("limit".into()).unwrap_or(&100);
    let offset = params.get("offset".into()).unwrap_or(&0);
    // Run custom querie to join `registros` table and `users` table
    let querie = format!(
        r#"SELECT registros.id, registros.rut, registros.fecha, registros.salida, registros.motivo, users.nombre, users.apellido_1, users.apellido_2, users.correo_uai, users.rol FROM registros JOIN users ON registros.rut = users.rut ORDER BY registros.fecha DESC LIMIT {} OFFSET {};"#,
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

// RegistroNew is the json body that we will receive when registering a new registro
#[derive(Serialize, Deserialize, ToSchema)]
pub struct RegistroNew {
    pub rut: String,
    pub salida: bool,
    pub motivo: String,
}

/// Register a new reigstro into the DB
#[utoipa::path(
    post,
    tag = "Registros",
    path = "/registros",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    request_body = RegistroNew
)]
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

/// Get last registro of a rut
#[utoipa::path(
   get,
    path = "/registros/last/{rut}",
    tag = "Registros",
    responses(
        (status = 200, description = "Success", body = [RegistroModel], examples((
            "Demo" = (description = "Long description",
                value = json!({
                    "id": 1,
                    "rut": "12345678-9",
                    "fecha": "2021-05-31T15:00:00-03:00",
                    "salida": true,
                    "motivo": "salida"
                })
            )),
        )),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    params(
        ("rut" = String, Path, description = "Rut of the user", example = "123456789")
    ),
    security(
        ("auth-cookie" = [])
    )
)]
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
    return Err((StatusCode::BAD_REQUEST, "Rut invalido".into()));
}

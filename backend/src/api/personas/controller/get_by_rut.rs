use crate::{
    api::utils::is_valid_num_rut,
    database::entities::{personas::Entity as Personas, registros},
    database::{entities::personas, pool::Pool},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use serde_json::Value;
use std::sync::Arc;

use crate::api::utils::internal_error;

// We extract the rut as a path `/:rut`
pub async fn get_persona_by_rut(
    State(pool): State<Arc<Pool>>,
    Path(rut): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Check if the rut is valid
    if !is_valid_num_rut(&rut) {
        return Err((StatusCode::NOT_FOUND, "".into()));
    }
    // Querie the rut
    let querie = Personas::find_by_id(rut).filter(personas::Column::IsDisabled.eq(false));
    let persona_row = querie.one(pool.get_db()).await.map_err(internal_error)?;

    // Return OK + Json(Persona) if exists or NO CONTENT if it doesn't
    if let Some(persona_row) = persona_row {
        // We join the last "registro" to the "persona" row
        let rut = &persona_row.rut;
        let last_registro = get_last_registro(rut, pool.clone()).await?;
        let mut val = serde_json::to_value(&persona_row).unwrap();
        val["last_registro"] = serde_json::to_value(&last_registro).unwrap();

        // Return the persona with the last registro
        Ok(Json(val))
    } else {
        Err((StatusCode::NO_CONTENT, "".into()))
    }
}

pub async fn get_last_registro(
    rut: &String,
    pool: Arc<Pool>,
) -> Result<Option<registros::Model>, (StatusCode, String)> {
    let query = registros::Entity::find()
        .filter(registros::Column::Rut.eq(rut))
        .order_by_desc(registros::Column::Fecha)
        .one(pool.get_db())
        .await
        .map_err(internal_error)?;
    Ok(query)
}

use crate::{
    api::utils::is_valid_num_rut,
    database::entities::personas::Entity as Personas,
    database::{entities::personas, pool::Pool},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::sync::Arc;

use crate::api::utils::internal_error;

// We extract the rut as a path `/:rut`
pub async fn get_persona_by_rut(
    State(pool): State<Arc<Pool>>,
    Path(rut): Path<String>,
) -> Result<Json<personas::Model>, (StatusCode, String)> {
    // Check if the rut is valid
    if !is_valid_num_rut(&rut) {
        return Err((StatusCode::NOT_FOUND, "".into()));
    }
    // Querie the rut
    let querie = Personas::find_by_id(rut).filter(personas::Column::IsDisabled.eq(false));
    let persona_row = querie.one(pool.get_db()).await.map_err(internal_error)?;

    // Return OK + Json(Persona) if exists or NO CONTENT if it doesn't
    if let Some(persona_row) = persona_row {
        Ok(Json(persona_row))
    } else {
        Err((StatusCode::NO_CONTENT, "".into()))
    }
}

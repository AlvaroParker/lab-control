use crate::{
    api::utils::is_valid_num_rut,
    database::entities::{registros, users::Entity as Users},
    database::{entities::users, pool::Pool},
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
pub async fn get_user_by_rut(
    State(pool): State<Arc<Pool>>,
    Path(rut): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    // Check if the rut is valid
    if !is_valid_num_rut(&rut) {
        return Err((StatusCode::NOT_FOUND, "".into()));
    }
    // Querie the rut
    let querie = Users::find_by_id(rut).filter(users::Column::IsDisabled.eq(false));
    let user_row = querie.one(pool.get_db()).await.map_err(internal_error)?;

    // Return OK + Json(User) if exists or NO CONTENT if it doesn't
    if let Some(user_row) = user_row {
        // We join the last "registro" to the "user" row
        let rut = &user_row.rut;
        let last_registro = get_last_registro(rut, pool.clone()).await?;
        let mut val = serde_json::to_value(&user_row).unwrap();
        val["last_registro"] = serde_json::to_value(&last_registro).unwrap();

        // Return the user with the last registro
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

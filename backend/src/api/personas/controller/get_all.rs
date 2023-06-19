use crate::{
    database::entities::personas::Entity as Personas,
    database::{entities::personas, pool::Pool},
};
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::sync::Arc;

use crate::api::utils::internal_error;
pub async fn get_all_personas(
    State(pool): State<Arc<Pool>>,
) -> Result<Json<Vec<personas::Model>>, (StatusCode, String)> {
    // Generate querie
    let querie = Personas::find().filter(personas::Column::IsDisabled.eq(false));

    // Run querie
    let personas_rows = querie.all(pool.get_db()).await.map_err(internal_error)?;

    Ok(Json(personas_rows))
}

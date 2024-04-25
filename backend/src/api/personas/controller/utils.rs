use crate::api::utils::internal_error;
use crate::database::entities::personas;
use crate::{database::entities::personas::Entity as Personas, database::pool::Pool};
use axum::http::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::sync::Arc;

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

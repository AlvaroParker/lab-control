use crate::{
    database::entities::users::Entity as Users,
    database::{entities::users, pool::Pool},
};
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::sync::Arc;

use crate::api::utils::internal_error;
// Get all users
pub async fn get_all_users(
    State(pool): State<Arc<Pool>>,
) -> Result<Json<Vec<users::Model>>, (StatusCode, String)> {
    // Generate querie
    let querie = Users::find().filter(users::Column::IsDisabled.eq(false));

    // Run querie
    let users_rows = querie.all(pool.get_db()).await.map_err(internal_error)?;

    Ok(Json(users_rows))
}

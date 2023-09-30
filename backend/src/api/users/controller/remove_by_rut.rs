use crate::{
    api::utils::is_valid_num_rut,
    database::entities::users::{self, Entity as Users},
    database::pool::Pool,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
use std::sync::Arc;

use crate::api::utils::internal_error;

// We extract the rut from the URI `/:rut`
pub async fn remove_user_by_rut(
    State(pool): State<Arc<Pool>>,
    Path(rut): Path<String>,
) -> Result<String, (StatusCode, String)> {
    // Check if the rut is valid
    if !is_valid_num_rut(&rut) {
        return Err((StatusCode::NOT_FOUND, "".into()));
    }

    // Create and execute the DELETE querie
    let querie = Users::find_by_id(rut)
        .one(pool.get_db())
        .await
        .map_err(internal_error)?;

    // Check if the querie was successfull, return status code OK if it was,
    // return NOT FOUND if it wasn't
    if let Some(user) = querie {
        let mut active_user: users::ActiveModel = user.into();
        active_user.is_disabled = ActiveValue::Set(true);
        active_user
            .update(pool.get_db())
            .await
            .map_err(internal_error)?;
        Ok("Person successfully deleted".into())
    } else {
        Err((StatusCode::NOT_FOUND, "Person not found".into()))
    }
}

use std::sync::Arc;

use axum::extract::Path;
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, EntityTrait, Set};

use crate::api::utils::{conflict_err, internal_error};
use crate::database::entities::roles::{self, Entity as RolesDB};
use crate::database::pool::Pool;

use super::models::Rol;

pub async fn get_all(
    State(pool): State<Arc<Pool>>,
) -> Result<Json<Vec<roles::Model>>, (StatusCode, String)> {
    if let Ok(motivos) = RolesDB::find().all(pool.get_db()).await {
        Ok(Json(motivos))
    } else {
        Err((StatusCode::NOT_FOUND, "No rols found".to_string()))
    }
}

// Todo handle duplicate rol entry
pub async fn post_rol(
    State(pool): State<Arc<Pool>>,
    Json(rol): Json<Rol>,
) -> Result<(), (StatusCode, String)> {
    let mut query = roles::ActiveModel::new();
    query.rol = Set(rol.rol);
    query.id = NotSet;

    query
        .insert(pool.get_db())
        .await
        .map_err(|err| conflict_err(err, "Rol already exists"))?;
    Ok(())
}

// Todo handle Database errors
pub async fn delete_rol(
    State(pool): State<Arc<Pool>>,
    Path(id): Path<i32>,
) -> Result<(), (StatusCode, String)> {
    let rows = RolesDB::delete_by_id(id)
        .exec(pool.get_db())
        .await
        .map_err(internal_error)?;
    if rows.rows_affected == 0 {
        return Err((StatusCode::NOT_FOUND, "Rol not found".into()));
    }
    Ok(())
}

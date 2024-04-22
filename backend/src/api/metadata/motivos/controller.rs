use std::sync::Arc;

use axum::extract::Path;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, EntityTrait, Set};
use serde::{Deserialize, Serialize};

use crate::api::utils::{conflict_err, internal_error};
use crate::database::entities::motivos::{self, Entity as MotivosDB};

use crate::database::pool::Pool;

use axum::{extract::State, http::StatusCode, Json};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Motivo {
    motivo: String,
}

pub async fn get_all(
    State(pool): State<Arc<Pool>>,
) -> Result<Json<Vec<motivos::Model>>, (StatusCode, String)> {
    if let Ok(motivos) = MotivosDB::find().all(pool.get_db()).await {
        Ok(Json(motivos))
    } else {
        Err((StatusCode::NOT_FOUND, "Motivos not found".to_string()))
    }
}

pub async fn post_motivo(
    State(pool): State<Arc<Pool>>,
    Json(motivo): Json<Motivo>,
) -> Result<(), (StatusCode, String)> {
    let mut query = motivos::ActiveModel::new();
    if motivo.motivo.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Empty motivo".to_string()));
    }
    query.motivo = Set(motivo.motivo);
    query.id = NotSet;

    query
        .insert(pool.get_db())
        .await
        .map_err(|err| conflict_err(err, "Motivo already exists"))?;
    Ok(())
}

pub async fn delete_motivo(
    State(pool): State<Arc<Pool>>,
    Path(id): Path<i32>,
) -> Result<(), (StatusCode, String)> {
    let rows = MotivosDB::delete_by_id(id)
        .exec(pool.get_db())
        .await
        .map_err(internal_error)?;

    if rows.rows_affected == 0 {
        return Err((StatusCode::NOT_FOUND, "Motivo not found".into()));
    }
    Ok(())
}

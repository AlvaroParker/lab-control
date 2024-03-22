use std::sync::Arc;

use axum::extract::Path;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, EntityTrait, Set};
use serde::{Deserialize, Serialize};

use crate::api::utils::internal_error;
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
        Err((
            StatusCode::NOT_FOUND,
            "No se encontraron motivos".to_string(),
        ))
    }
}

pub async fn post_motivo(
    State(pool): State<Arc<Pool>>,
    Json(motivo): Json<Motivo>,
) -> Result<(), (StatusCode, String)> {
    let mut query = motivos::ActiveModel::new();
    query.motivo = Set(motivo.motivo);
    query.id = NotSet;

    query.insert(pool.get_db()).await.map_err(internal_error)?;
    Ok(())
}

pub async fn delete_motivo(
    State(pool): State<Arc<Pool>>,
    Path(id): Path<i32>,
) -> Result<(), (StatusCode, String)> {
    MotivosDB::delete_by_id(id)
        .exec(pool.get_db())
        .await
        .map_err(internal_error)?;
    Ok(())
}

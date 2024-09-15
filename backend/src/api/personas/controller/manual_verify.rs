use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use chrono::{Local, Utc};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::{
    api::utils::{internal_error, is_valid_num_rut},
    database::{
        entities::{
            personas::{self, Entity as Personas},
            registros,
        },
        pool::Pool,
    },
};

#[derive(Serialize, Deserialize)]
pub struct Registro {
    pub salida: bool,
    pub motivo: String,
    pub rut: String,
}

pub async fn manual_verify_persona(
    State(pool): State<Arc<Pool>>,
    Json(registro): Json<Registro>,
) -> Result<Json<personas::Model>, (StatusCode, String)> {
    // Check if the rut is valid
    if !is_valid_num_rut(&registro.rut) {
        return Err((StatusCode::NOT_FOUND, "".into()));
    }
    // Querie the rut
    let querie = Personas::find_by_id(&registro.rut).filter(personas::Column::IsDisabled.eq(false));
    let persona_row = querie.one(pool.get_db()).await.map_err(internal_error)?;

    // Return OK + Json(Persona) if exists or NO CONTENT if it doesn't
    if let Some(persona_row) = persona_row {
        let now = Local::now();
        let offset_in_sec = now.offset();

        let now = Utc::now().with_timezone(offset_in_sec);
        let new_registro = registros::ActiveModel {
            id: ActiveValue::NotSet,
            salida: ActiveValue::set(registro.salida),
            rut: ActiveValue::set(registro.rut),
            fecha: ActiveValue::set(now),
            motivo: ActiveValue::set(registro.motivo),
        };
        new_registro
            .insert(pool.get_db())
            .await
            .map_err(internal_error)?;
        Ok(Json(persona_row))
    } else {
        Err((StatusCode::NO_CONTENT, "".into()))
    }
}

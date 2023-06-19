use crate::api::utils::internal_error;
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
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// The POST json body takes `EditPersona` parameters as arguments.
#[derive(Serialize, Deserialize)]
pub struct EditPersona {
    pub nombre: Option<String>,
    pub apellido_1: Option<String>,
    pub apellido_2: Option<String>,
    pub correo_uai: Option<String>,
    pub rut: Option<String>,
    pub rol: Option<String>,
}

// POST request, extract JSON from request Serializing it to an EditPersona struct
pub async fn edit_persona_by_rut(
    State(pool): State<Arc<Pool>>,
    Path(rut): Path<String>,
    Json(edit_persona): Json<EditPersona>,
) -> Result<(), (StatusCode, String)> {
    if !is_valid_num_rut(&rut) {
        return Err((StatusCode::NOT_FOUND, "".into()));
    }

    let persona = Personas::find_by_id(rut.clone())
        .one(pool.get_db())
        .await
        .map_err(internal_error)?;

    if let Some(persona) = persona {
        let mut persona: personas::ActiveModel = persona.into();

        update_row(&mut persona, edit_persona);

        let res = Personas::update_many()
            .set(persona)
            .filter(personas::Column::Rut.eq(rut))
            .exec(pool.get_db())
            .await
            .map_err(internal_error)?;
        eprintln!("WENNNNNNNNNNNNNNNNNA");
        dbg!(res);

        Ok(())
    } else {
        Err((StatusCode::NO_CONTENT, "".into()))
    }
}

// Update row function based on values passed on the JSON body
fn update_row(persona: &mut personas::ActiveModel, edit_persona: EditPersona) {
    if let Some(nombre) = edit_persona.nombre {
        persona.nombre = Set(nombre);
    }
    if let Some(apellido_1) = edit_persona.apellido_1 {
        persona.apellido_1 = Set(apellido_1);
    }
    if let Some(apellido_2) = edit_persona.apellido_2 {
        persona.apellido_2 = Set(apellido_2);
    }
    if let Some(correo_uai) = edit_persona.correo_uai {
        persona.correo_uai = Set(correo_uai);
    }
    if let Some(rut) = edit_persona.rut {
        persona.rut = Set(rut);
    }
    if let Some(rol) = edit_persona.rol {
        persona.rol = Set(rol);
    };
}

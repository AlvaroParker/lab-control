use crate::api::utils::internal_error;
use crate::{
    api::utils::is_valid_num_rut,
    database::entities::users::Entity as Users,
    database::{entities::users, pool::Pool},
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

// The POST json body takes `EditUser` parameters as arguments.
#[derive(Serialize, Deserialize)]
pub struct EditUser {
    pub nombre: Option<String>,
    pub apellido_1: Option<String>,
    pub apellido_2: Option<String>,
    pub correo_uai: Option<String>,
    pub rut: Option<String>,
    pub rol: Option<String>,
}

/// POST request, extract JSON from request Serializing it to an EditUser struct
pub async fn edit_user_by_rut(
    State(pool): State<Arc<Pool>>,
    Path(rut): Path<String>,
    Json(edit_user): Json<EditUser>,
) -> Result<(), (StatusCode, String)> {
    if !is_valid_num_rut(&rut) {
        return Err((StatusCode::NOT_FOUND, "Not a valid rut".into()));
    }

    let user = Users::find_by_id(&rut)
        .one(pool.get_db())
        .await
        .map_err(|err| {
            if let sea_orm::DbErr::RecordNotFound(e) = err {
                tracing::error!("User not found: {:?}", e);
                (StatusCode::NOT_FOUND, "User not found".into())
            } else {
                internal_error(err)
            }
        })?;

    if let Some(user) = user {
        let mut user: users::ActiveModel = user.into();

        update_row(&mut user, edit_user);

        let res = Users::update_many()
            .set(user)
            .filter(users::Column::Rut.eq(rut))
            .exec(pool.get_db())
            .await
            .map_err(internal_error)?;
        tracing::info!("Updated user: {:?}", res);

        Ok(())
    } else {
        Err((StatusCode::NO_CONTENT, "".into()))
    }
}

// Update row function based on values passed on the JSON body
fn update_row(user: &mut users::ActiveModel, edit_user: EditUser) {
    if let Some(nombre) = edit_user.nombre {
        user.nombre = Set(nombre);
    }
    if let Some(apellido_1) = edit_user.apellido_1 {
        user.apellido_1 = Set(apellido_1);
    }
    if let Some(apellido_2) = edit_user.apellido_2 {
        user.apellido_2 = Set(apellido_2);
    }
    if let Some(correo_uai) = edit_user.correo_uai {
        user.correo_uai = Set(correo_uai);
    }
    if let Some(rut) = edit_user.rut {
        user.rut = Set(rut);
    }
    if let Some(rol) = edit_user.rol {
        user.rol = Set(rol);
    };
}

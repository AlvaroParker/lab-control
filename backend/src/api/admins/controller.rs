use std::sync::Arc;

use crate::{api::utils::create_jwt, database::entities::admins::Entity as Admin};
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, EntityTrait};
use serde::{Deserialize, Serialize};

use crate::{
    api::utils::internal_error,
    database::{entities::admins, pool::Pool},
};

#[derive(Serialize, Deserialize)]
pub struct RequestAdmin {
    pub nombre: String,
    pub apellido_1: String,
    pub apellido_2: String,
    pub email: String,
    pub pswd: String,
}
#[derive(Serialize, Deserialize)]
pub struct ResponseAdmin {
    pub nombre: String,
    pub apellido_1: String,
    pub apellido_2: String,
    pub email: String,
    pub token: String,
}
use sea_orm::entity::ActiveValue::Set;

pub async fn create_user(
    State(pool): State<Arc<Pool>>,
    Json(admin): Json<RequestAdmin>,
) -> Result<Json<ResponseAdmin>, (StatusCode, String)> {
    let jwt = create_jwt()?;
    let new_admin = admins::ActiveModel {
        nombre: Set(admin.nombre),
        apellido_1: Set(admin.apellido_1),
        apellido_2: Set(admin.apellido_2),
        email: Set(admin.email),
        pswd: Set(hash_password(admin.pswd)?),
    }
    .insert(pool.get_db())
    .await
    .map_err(internal_error)?;

    Ok(Json(ResponseAdmin {
        nombre: new_admin.nombre,
        apellido_1: new_admin.apellido_1,
        apellido_2: new_admin.apellido_2,
        email: new_admin.email,
        token: jwt,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct LoginAdmin {
    email: String,
    pswd: String,
}
pub async fn login(
    State(pool): State<Arc<Pool>>,
    Json(request_admin): Json<LoginAdmin>,
) -> Result<Json<ResponseAdmin>, (StatusCode, String)> {
    let querie = Admin::find_by_id(request_admin.email)
        .one(pool.get_db())
        .await
        .map_err(internal_error)?;
    if let Some(saved_admin) = querie {
        let hash = saved_admin.pswd;
        let password = request_admin.pswd;
        let verified = verify_password(password, hash.as_str())?;
        if verified {
            let jwt = create_jwt()?;
            return Ok(Json(ResponseAdmin {
                nombre: saved_admin.nombre,
                apellido_1: saved_admin.apellido_1,
                apellido_2: saved_admin.apellido_2,
                email: saved_admin.email,
                token: jwt,
            }));
        }
    }
    Err((StatusCode::UNAUTHORIZED, "Unauthorized".into()))
}

pub async fn auth() -> Result<(), ()> {
    Ok(())
}

pub fn hash_password(password: String) -> Result<String, (StatusCode, String)> {
    bcrypt::hash(password, 10).map_err(internal_error)
}

pub fn verify_password(password: String, hash: &str) -> Result<bool, (StatusCode, String)> {
    bcrypt::verify(password, hash).map_err(internal_error)
}

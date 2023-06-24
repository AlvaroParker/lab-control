use std::{sync::Arc, time::Duration};

use crate::{api::utils::handle_cookie_err, database::entities::admins::Entity as Admin};
use async_session::{Session, SessionStore};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
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
#[derive(Serialize, Deserialize, Clone)]
pub struct ResponseAdmin {
    pub nombre: String,
    pub apellido_1: String,
    pub apellido_2: String,
    pub email: String,
    pub cookie: Option<String>,
}
use sea_orm::entity::ActiveValue::Set;

pub async fn create_user(
    State(pool): State<Arc<Pool>>,
    Json(admin): Json<RequestAdmin>,
) -> Result<Json<ResponseAdmin>, (StatusCode, String)> {
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
    let admin = ResponseAdmin {
        nombre: new_admin.nombre,
        apellido_1: new_admin.apellido_1,
        apellido_2: new_admin.apellido_2,
        email: new_admin.email,
        cookie: None,
    };
    let admin = create_cookie(admin, pool).await?;

    Ok(Json(admin))
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
            let admin = ResponseAdmin {
                nombre: saved_admin.nombre,
                apellido_1: saved_admin.apellido_1,
                apellido_2: saved_admin.apellido_2,
                email: saved_admin.email,
                cookie: None,
            };

            let admin = create_cookie(admin, pool).await?;

            return Ok(Json(admin));
        }
    }
    Err((StatusCode::UNAUTHORIZED, "Unauthorized".into()))
}

pub async fn logout(
    State(pool): State<Arc<Pool>>,
    headers: HeaderMap,
) -> Result<String, (StatusCode, String)> {
    let cookie = match headers.get("cookie-auth") {
        Some(cookie) => cookie,
        None => return Err((StatusCode::BAD_REQUEST, "Missing auth cookie".into())),
    };
    let cookie_id = match cookie.to_str() {
        Ok(cookie_id) => cookie_id.to_string(),
        Err(_e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            ))
        }
    };
    let session = match pool.get_store().load_session(cookie_id).await {
        Ok(session) => match session {
            Some(session) => session,
            None => return Ok("Logout".into()),
        },
        Err(err) => return Err(internal_error(err.root_cause())),
    };
    pool.get_store()
        .destroy_session(session)
        .await
        .map_err(|e| internal_error(e.root_cause()))?;
    Ok("Logout".into())
}

async fn create_cookie(
    mut admin: ResponseAdmin,
    pool: Arc<Pool>,
) -> Result<ResponseAdmin, (StatusCode, String)> {
    let mut session = Session::new();
    session.insert("admin", &admin).map_err(internal_error)?;
    session.expire_in(Duration::from_secs(60) * 60 * 24);

    let cookie = pool
        .get_store()
        .store_session(session)
        .await
        .map_err(handle_cookie_err)?;

    if let Some(cookie) = cookie {
        admin.cookie = Some(cookie);
        Ok(admin)
    } else {
        Err((StatusCode::UNAUTHORIZED, "Unauthorized".into()))
    }
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

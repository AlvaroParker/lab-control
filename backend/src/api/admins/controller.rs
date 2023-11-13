use std::{sync::Arc, time::Duration};

use crate::{api::utils::handle_cookie_err, database::entities::admins::Entity as Admin};
use async_session::{Session, SessionStore};
use axum::{
    extract::State,
    http::{header::SET_COOKIE, Response, StatusCode},
    response::IntoResponse,
    Json,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    api::utils::internal_error,
    database::{entities::admins, pool::Pool},
};

// Struct representing the JSON that user will send to the webserver
#[derive(Serialize, Deserialize)]
pub struct RequestAdmin {
    pub nombre: String,
    pub apellido_1: String,
    pub apellido_2: String,
    pub email: String,
    pub pswd: String,
}
// Struct representing JSON that the server will response
#[derive(Serialize, Deserialize, Clone)]
pub struct ResponseAdmin {
    pub nombre: String,
    pub apellido_1: String,
    pub apellido_2: String,
    pub email: String,
    pub cookie: Option<String>,
}
use sea_orm::entity::ActiveValue::Set;

// Aka signin. The user must provide a valid `RequestAdmin` JSON body
pub async fn create_user(
    State(pool): State<Arc<Pool>>,
    Json(admin): Json<RequestAdmin>,
) -> Result<Json<ResponseAdmin>, (StatusCode, String)> {
    // Create a new admin SQL row
    let new_admin = admins::ActiveModel {
        nombre: Set(admin.nombre),
        apellido_1: Set(admin.apellido_1),
        apellido_2: Set(admin.apellido_2),
        email: Set(admin.email),
        pswd: Set(hash_password(admin.pswd)?),
    }
    // Insert the row to the DB
    .insert(pool.get_db())
    .await
    // If there's any error, send a 500 internal error to the client
    .map_err(user_exists)?;
    // From the new created user, create a ResponseAdmin struct
    let mut admin = ResponseAdmin {
        nombre: new_admin.nombre,
        apellido_1: new_admin.apellido_1,
        apellido_2: new_admin.apellido_2,
        email: new_admin.email,
        cookie: None,
    };
    // Create a cookie for the user
    create_cookie(&mut admin, pool).await?;

    // Send the ResponseAdmin struct to the client
    Ok(Json(admin))
}

// Login struct, the client must provide this field in order to login
#[derive(Serialize, Deserialize)]
pub struct LoginAdmin {
    email: String,
    pswd: String,
}
// The client must provide a valid `LoginAdmin` JSON body
pub async fn login(
    State(pool): State<Arc<Pool>>,
    Json(request_admin): Json<LoginAdmin>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Find the admin in the DB by email
    let querie = Admin::find_by_id(request_admin.email)
        .one(pool.get_db())
        .await
        .map_err(internal_error)?;
    // If the admin exists, verify the password
    if let Some(saved_admin) = querie {
        // Get the password hash from the DB
        let hash = saved_admin.pswd;
        // Get the password from the request
        let password = request_admin.pswd;
        // Verify the password
        let verified = verify_password(password, hash.as_str())?;
        // If the password is correct, create a cookie for the admin
        if verified {
            let mut admin = ResponseAdmin {
                nombre: saved_admin.nombre,
                apellido_1: saved_admin.apellido_1,
                apellido_2: saved_admin.apellido_2,
                email: saved_admin.email,
                cookie: None,
            };

            create_cookie(&mut admin, pool).await?;
            // Send the ResponseAdmin struct to the client
            let mut cookie = Cookie::new("auth-cookie", admin.cookie.clone().unwrap());
            cookie.set_path("/");
            let response = Response::builder()
                .header(SET_COOKIE, cookie.to_string())
                .header("Content-Type", "application/json")
                .body(json!(admin).to_string())
                .unwrap();

            return Ok(response);
        }
    }
    // Fallback if the admin doesn't exist or the password is incorrect
    Err((StatusCode::UNAUTHORIZED, "Unauthorized".into()))
}

// Logout, delete the active session from the database.
pub async fn logout(
    State(pool): State<Arc<Pool>>,
    jar: CookieJar,
) -> Result<String, (StatusCode, String)> {
    // Get the cookie from the request, if any
    let cookie = match jar.get("auth-cookie") {
        Some(cookie) => cookie.value(),
        None => return Err((StatusCode::BAD_REQUEST, "Missing auth cookie".into())),
    };
    // Load the session from the DB
    let session = match pool.get_store().load_session(cookie.into()).await {
        Ok(session) => match session {
            Some(session) => session,
            None => return Ok("Logout".into()),
        },
        Err(err) => return Err(internal_error(err.root_cause())),
    };
    // Destroy the session found
    pool.get_store()
        .destroy_session(session)
        .await
        .map_err(|e| internal_error(e.root_cause()))?;
    // Return 200 OK logout sucessful
    Ok("Logout".into())
}

// Create a cookie for the given `Admin`
async fn create_cookie(
    admin: &mut ResponseAdmin,
    pool: Arc<Pool>,
) -> Result<(), (StatusCode, String)> {
    // Create a new session
    let mut session = Session::new();
    // Insert admin data to the session cookie
    session.insert("admin", &admin).map_err(internal_error)?;
    // Set expiration date in 24 hours
    session.expire_in(Duration::from_secs(60) * 60 * 24);

    // Store the session in the DB
    let cookie = pool
        .get_store()
        .store_session(session)
        .await
        .map_err(handle_cookie_err)?;

    // If the cookie was successfully created, set the cookie in the `ResponseAdmin` struct
    // else return 500 internal server error
    if let Some(cookie) = cookie {
        admin.cookie = Some(cookie);
        Ok(())
    } else {
        Err((StatusCode::UNAUTHORIZED, "Unauthorized".into()))
    }
}

pub async fn auth() -> Result<(), ()> {
    Ok(())
}

// Hash a password using bcrypt
pub fn hash_password(password: String) -> Result<String, (StatusCode, String)> {
    bcrypt::hash(password, 10).map_err(internal_error)
}

// Verify a password using bcrypt
pub fn verify_password(password: String, hash: &str) -> Result<bool, (StatusCode, String)> {
    bcrypt::verify(password, hash).map_err(internal_error)
}

// Handle user exists in `insert` query
pub fn user_exists(err: DbErr) -> (StatusCode, String) {
    if let DbErr::Query(sea_orm::RuntimeErr::SqlxError(sqlx::Error::Database(e))) = err {
        // Try to cast Box<dyn Error> to PgDatabaseError
        if let Ok(err) = e.try_downcast::<sqlx::postgres::PgDatabaseError>() {
            if err.code() == "23505" {
                return (StatusCode::BAD_REQUEST, "User already exists".to_string());
            }
        }
    }
    // Return internal error if the error is not a duplicate key error
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal Server Error".to_string(),
    )
}

use std::{sync::Arc, time::Duration};

use crate::{api::utils::handle_cookie_err, database::entities::admins::Entity as Admin};
use async_session::{Session, SessionStore};
use axum::{
    extract::State,
    http::{header::SET_COOKIE, Response, StatusCode},
    response::IntoResponse,
    Json,
};
use axum_extra::extract::CookieJar;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

use crate::{
    api::utils::internal_error,
    database::{entities::admins, pool::Pool},
};

// Struct representing the JSON that admin will send to the webserver
// This is used to create a new admin
#[derive(Serialize, Deserialize, ToSchema)]
pub struct RequestAdmin {
    pub nombre: String,
    pub apellido_1: String,
    pub apellido_2: String,
    pub email: String,
    pub pswd: String,
}
// Struct representing the ADMIN JSON that the server will response
#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct ResponseAdmin {
    #[schema(example = "Bruce")]
    pub nombre: String,
    #[schema(example = "Banner")]
    pub apellido_1: String,
    #[schema(example = "The Hulk")]
    pub apellido_2: String,
    #[schema(example = "bbanner@alumnos.uai.cl")]
    pub email: String,
    #[schema(nullable)]
    pub cookie: Option<String>,
}
use sea_orm::entity::ActiveValue::Set;

/// Aka signin. The admin must provide a valid `RequestAdmin` JSON body
#[utoipa::path(
   post,
    path = "/admin/signin",
    tag = "Admins",
    responses(
        (status = 200, description = "Success", body = ResponseAdmin, examples((
            "Demo" = (description = "Long description",
                value = json!({
                    "nombre": "Bruce",
                    "apellido_1": "Banner",
                    "apellido_2": "The Hulk",
                    "email": "bbanner@alumnos.uai.cl",
                })
            )),
        )),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    request_body = RequestAdmin,
    security(
        ("auth-cookie" = [])
    )
)]
pub async fn create_admin(
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
    .map_err(admin_exists)?;
    // From the newly created admin, create a ResponseAdmin struct
    let mut admin = ResponseAdmin {
        nombre: new_admin.nombre,
        apellido_1: new_admin.apellido_1,
        apellido_2: new_admin.apellido_2,
        email: new_admin.email,
        cookie: None,
    };
    // Create a cookie for the admin
    create_cookie(&mut admin, pool).await?;

    // Send the ResponseAdmin struct to the client
    Ok(Json(admin))
}

// Login struct, the client must provide this field in order to login
#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginAdmin {
    email: String,
    pswd: String,
}
/// Login endpoint, the client must provide a valid `LoginAdmin` JSON body
#[utoipa::path(
    post,
    path = "/admin/login",
    tag = "Admins",
    request_body = LoginAdmin,
    responses(
        (status = 200, description = "Success", body = ResponseAdmin, examples((
            "Demo" = (description = "Long description",
                value = json!({
                    "nombre": "Bruce",
                    "apellido_1": "Banner",
                    "apellido_2": "The Hulk",
                    "email": "bbanner@alumnos.uai.cl",
                    "cookie": "eyJhbGciOiJIUzI1NiIsInR"
                })
            )),
        )),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    )
)]
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
            let body = axum::body::Body::from(json!(admin).to_string());
            // Send the ResponseAdmin struct to the client
            let cookie = admin.cookie.as_ref().unwrap().clone();
            let response = Response::builder()
                .header(SET_COOKIE, format!("auth-cookie={}", cookie))
                .header("Content-Type", "application/json")
                .body(body)
                .unwrap();

            return Ok(response);
        }
    }
    // Fallback if the admin doesn't exist or the password is incorrect
    Err((StatusCode::UNAUTHORIZED, "Unauthorized".into()))
}

/// Logout endpoint, the client must provide a cookie with his auth token
#[utoipa::path(
    post,
    path = "/admin/logout",
    tag = "Admins",
    responses(
        (status = 200, description = "Success", example = "Logout"),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        (), // <-- Make optional authentication
        ("auth-cookie" = [])
    )
)]
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

// Handle admin exists in `insert` query
pub fn admin_exists(err: DbErr) -> (StatusCode, String) {
    match err {
        DbErr::Query(sea_orm::RuntimeErr::SqlxError(sqlx::Error::Database(e))) => {
            // Try to cast Box<dyn Error> to PgDatabaseError
            match e.try_downcast::<sqlx::postgres::PgDatabaseError>() {
                Ok(err) => {
                    // If the error code is 23505, it means that the admin already exists
                    if err.code() == "23505" {
                        return (StatusCode::BAD_REQUEST, "Admin already exists".to_string());
                    }
                }
                Err(_) => {}
            }
        }
        _ => {}
    };
    // Return internal error if the error is not a duplicate key error
    return (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal Server Error".to_string(),
    );
}

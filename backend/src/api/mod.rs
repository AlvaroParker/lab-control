mod admins;
mod guard;
mod registros;
pub mod routes;
mod users;
pub(crate) mod utils;

use utoipa::openapi::security::ApiKey;
use utoipa::openapi::security::ApiKeyValue;
use utoipa::openapi::security::SecurityScheme;
use utoipa::Modify;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        admins::controller::create_admin,
        admins::controller::login,
        admins::controller::logout,

        registros::controller::get_all,
        registros::controller::get_last,
        registros::controller::registrar_rut,
    ),
    components(schemas(
        admins::controller::ResponseAdmin,
        admins::controller::LoginAdmin,
        admins::controller::RequestAdmin,

        registros::controller::RegistroAlumno,
        registros::controller::RegistroNew,

        crate::database::entities::registros::Model,
        crate::database::entities::users::Model,
    )),
    tags(
        (name = "Admins", description = "Admins"),
        (name = "Registros", description = "Registros"),
    ),
    modifiers(&SecurityAddon),
    servers(
        (url = "https://192.168.68.117:8001/api")
    )
)]
pub struct ApiDocs;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = &mut openapi.components.as_mut() {
            components.add_security_scheme(
                "auth-cookie",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("auth-cookie".to_string()))),
            )
        }
    }
}

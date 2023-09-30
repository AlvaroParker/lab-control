use crate::{
    database::entities::users::{self, Entity as Users},
    database::{entities::registros, pool::Pool},
};

use super::utils::{Action, Body};
use crate::api::utils::internal_error;
use axum::{extract::State, http::StatusCode, Json};
use chrono::{Local, Utc};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    io::{self, ErrorKind},
    sync::Arc,
};
use tungstenite::{connect, Message};

// This struct is used on Verify to check if we should add `salida` or `entrada` to the Db
#[derive(Serialize, Deserialize)]
pub struct Salida {
    pub salida: bool,
    pub motivo: String,
}

pub async fn verify_user(
    State(pool): State<Arc<Pool>>,
    Json(salida): Json<Salida>,
) -> Result<Json<users::Model>, (StatusCode, String)> {
    // Querie to get all prints paths
    let querie = Users::find()
        .select_only()
        .column(users::Column::PrintPath)
        .into_tuple();

    // Run querie
    let paths: Vec<String> = querie.all(pool.get_db()).await.map_err(internal_error)?;

    // Check if user print matches any of the prints in the DB
    let response = verify_print(paths).map_err(internal_error)?;

    // If there's a match, take the path
    let matched_print_path = match response.split_whitespace().collect::<Vec<&str>>().get(2) {
        Some(path) => path.to_string(),
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                "Fingerprint verification error".into(),
            ))
        }
    };

    // Find the corresponding user by PrintPath
    let querie_by_path = Users::find()
        .filter(users::Column::PrintPath.eq(matched_print_path))
        .filter(users::Column::IsDisabled.eq(false));

    // Run the querie
    let user = querie_by_path
        .one(pool.get_db())
        .await
        .map_err(internal_error)?;

    match user {
        Some(user) => {
            // Return user as json and ADD the registro to the Registro table
            let now = Local::now();
            let offset_in_sec = now.offset();

            let now = Utc::now().with_timezone(offset_in_sec);
            let new_registro = registros::ActiveModel {
                id: ActiveValue::NotSet,
                salida: ActiveValue::set(salida.salida),
                rut: ActiveValue::set(user.rut.clone()),
                fecha: ActiveValue::set(now),
                motivo: ActiveValue::set(salida.motivo),
            };
            new_registro
                .insert(pool.get_db())
                .await
                .map_err(internal_error)?;
            return Ok(Json(user));
        }
        // If there are no paths matching in the databse, return not found
        None => return Err((StatusCode::NOT_FOUND, "Person not found".into())),
    }
}

fn verify_print(paths: Vec<String>) -> Result<String, io::Error> {
    let body = Body {
        action: Action::Verify,
        paths,
    };
    let json_body = json!(body).to_string();

    let sock_ip = crate::SOCKET_IP.to_string();
    let sock_port = crate::SOCKET_PORT.to_string();

    let (mut socket, _response) = connect(format!("ws://{}:{}/socket", sock_ip, sock_port))
        .map_err(|err| {
            tracing::error!("Error while trying to connect to socket: {}", &err);
            io::Error::new(ErrorKind::ConnectionRefused, err)
        })?;
    socket
        .write_message(Message::text(json_body.clone()))
        .unwrap();
    let mut path = String::default();
    loop {
        let msg = match socket.read_message() {
            Ok(msg) => msg,
            Err(e) => {
                tracing::error!(
                    "Error while trying to read message from fingerprint socket: {}",
                    e
                );
                eprintln!("{}", e);
                break;
            }
        };
        if let Message::Text(msg) = msg {
            path = msg.clone();
        }
    }
    Ok(path)
}

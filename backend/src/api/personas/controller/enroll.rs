use super::utils::{Action, Body, NewPersona, Outer};
use crate::{
    api::{personas::controller::utils::rut_exists_or_email, utils::is_valid_num_rut},
    database::entities::personas,
    database::pool::Pool,
};
use axum::{
    extract::{
        ws::{self, close_code, CloseFrame, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
use sea_orm::ActiveModelTrait;
use serde_json::{from_str, json};
use std::{borrow::Cow, sync::Arc};
use tungstenite::Error as TunError;
use tungstenite::{connect, Message};

pub async fn enroll_persona(ws: WebSocketUpgrade, State(pool): State<Arc<Pool>>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, pool.clone()))
}

const INTERNAL_SERVER_ERROR: u16 = 4500;
const USER_EXISTS_ERR: u16 = 4000;
const INVALID_RUT_ERR: u16 = 4001;
const DB_INSERTION_ERR: u16 = 4002;
const FP_SENSOR_ERR: u16 = 4003;

async fn handle_socket(mut socket: WebSocket, pool: Arc<Pool>) {
    // Get the data from message from client
    let data = match socket.recv().await {
        Some(Ok(ws::Message::Text(data))) => data,
        _ => return,
    };

    // Validate that the message is a valid JSON of a new persona
    // check if the rut and email provided are valid a don't exist already
    let persona = match validate_message(data, pool.clone()).await {
        Ok(persona) => persona,
        Err(err) => {
            send_and_close(err, socket).await;
            return;
        }
    };
    // Enroll print sending a message to the fingerprint sensor socket
    let print = match enroll_print(&mut socket).await {
        Ok(print) => print,
        Err(e) => {
            // send_and_close((close_code::NORMAL, "Enrollment finished".into()), ws).await;
            let message = e.to_string();

            if socket
                .send(close_msg(FP_SENSOR_ERR, message))
                .await
                .is_err()
            {
                tracing::error!(
                    "Error sending ERROR message originated from fingerprint sensor read"
                )
            };
            return;
        }
    }; // Todo: handle the socket closing if the fingerprint panics

    // Get the path from the print response string
    let path = match print.split_whitespace().skip(1).next() {
        Some(s) => s.to_string(),
        None => {
            return;
        }
    };
    // Create the new persona struct using the Persona JSON provided by the client and
    // the path provided by the fingerprint sensor
    let json_persona = json!(Outer::new(persona, path));
    dbg!(&json_persona.to_string());

    // It never fails since is made from persona and path wich together make an ActiveModel
    // Create a new DB model with the json_persona object
    let persona_model = personas::ActiveModel::from_json(json_persona).unwrap();

    // Insert the new row
    if let Err(e) = persona_model.insert(pool.get_db()).await {
        let msg = close_msg(DB_INSERTION_ERR, "Error while saving new user".into());

        if let Err(socket_err) = socket.send(msg).await {
            tracing::error!(
                "Error ({}) sending database error ({}) to client",
                socket_err,
                e
            );
        };
        return;
    }
    if socket
        .send(close_msg(
            close_code::NORMAL,
            "Usuario successfully added".into(),
        ))
        .await
        .is_err()
    {
        tracing::error!("Error sending sucessfully added response to client");
    };
}

async fn enroll_print(ws: &mut WebSocket) -> Result<String, String> {
    let body = Body {
        action: Action::Enroll,
        paths: vec![],
    };
    let json_body = json!(body).to_string();
    let sock_ip = std::env::var("SOCKET_IP").unwrap_or("127.0.0.1".into());
    let sock_port = std::env::var("SOCKET_PORT").unwrap_or("5000".into());

    let (mut socket, _response) =
        connect(format!("ws://{}:{}/socket", sock_ip, sock_port)).map_err(|e| e.to_string())?;
    socket
        .write_message(Message::text(json_body.clone()))
        .map_err(|e| e.to_string())?;

    let mut path = String::default();
    loop {
        let msg = match socket.read_message() {
            Ok(msg) => msg,
            Err(e) => match e {
                TunError::ConnectionClosed => {
                    break;
                }
                _ => {
                    return Err(e.to_string());
                }
            },
        };

        if let Message::Text(msg) = msg {
            if !msg.contains("SUCCESS") {
                let ws_msg = ws::Message::Text(msg.clone());
                ws.send(ws_msg).await.map_err(|e| e.to_string())?;
            } else {
                path = msg
            }
        }
    }

    tracing::info!("Enroll path: {}", &path);
    path = path.trim_matches(char::from(0)).to_string();
    dbg!(&path);
    Ok(path)
}

fn close_msg(close_code: u16, reason: String) -> ws::Message {
    ws::Message::Close(Some(CloseFrame {
        code: close_code,
        reason: Cow::Owned(reason),
    }))
}

async fn send_and_close(err: (u16, String), mut socket: WebSocket) {
    let (error_code, error_message) = err;
    tracing::info!("{}", error_message);
    if let Err(send_error) = socket.send(close_msg(error_code, error_message)).await {
        println!("Failed to send close message: {:?}", send_error);
    }
}

async fn validate_message(data: String, pool: Arc<Pool>) -> Result<NewPersona, (u16, String)> {
    let persona: NewPersona = match from_str(&data) {
        Ok(persona) => persona,
        Err(e) => {
            return Err((DB_INSERTION_ERR, e.to_string()));
        }
    };

    if !is_valid_num_rut(&persona.rut) {
        return Err((INVALID_RUT_ERR, "Invalid rut".into()));
    }

    let exists = rut_exists_or_email(pool.clone(), &persona.rut, &persona.correo_uai).await;
    if let Err(_e) = exists {
        return Err((INTERNAL_SERVER_ERROR, "Internal server error".into()));
    } else if exists.unwrap() {
        return Err((USER_EXISTS_ERR, "Email or rut already registered".into()));
    }
    Ok(persona)
}

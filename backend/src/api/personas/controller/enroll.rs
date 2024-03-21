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
use std::{borrow::Cow, sync::Arc};

// Handle the new connection and upgrade to a WebSocket
pub async fn enroll_persona(ws: WebSocketUpgrade, State(pool): State<Arc<Pool>>) -> Response {
    ws.on_failed_upgrade(|err| {
        tracing::error!("Failed to upgrade connection: {}", err);
    })
    .on_upgrade(move |socket| handle_socket(socket, pool.clone()))
}

// The result of the enrollment is a JSON, this struct is used to deserialize the JSON
// if result is true, the enrollment was successful and the path is the path to the fingerprint
#[derive(serde::Deserialize)]
struct Sucess<'a> {
    result: bool,
    path: Option<&'a str>,
}

// Returning errors, this are implemented as well on the frontend
pub(crate) const INTERNAL_SERVER_ERROR: u16 = 4500;
pub(crate) const USER_EXISTS_ERR: u16 = 4000;
pub(crate) const INVALID_RUT_ERR: u16 = 4001;
pub(crate) const DB_INSERTION_ERR: u16 = 4002;
pub(crate) const FP_SENSOR_ERR: u16 = 4003;

// Handle a new socket connection from the client
async fn handle_socket(mut socket: WebSocket, pool: Arc<Pool>) {
    // Get the first message from the client, this should be a JSON with the new persona data
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

    // Create the new persona struct using the Persona JSON provided by the client and
    // the path provided by the fingerprint sensor
    let json_persona = serde_json::json!(Outer::new(persona, &print));

    // Create a new DB model with the json_persona object
    let persona_model = personas::ActiveModel::from_json(json_persona).unwrap(); // this never
                                                                                 // fails

    // Insert the new row
    if let Err(e) = persona_model.insert(pool.get_db()).await {
        let msg = close_msg(DB_INSERTION_ERR, "Error while saving new user".into());
        // Close the socket with DB_INSErtION_ERR message
        if let Err(socket_err) = socket.send(msg).await {
            // Log the error in the database
            tracing::error!(
                "Error ({}) sending database error ({}) to client",
                socket_err,
                e
            );
        };
        return;
    }
    // Close the socket normally
    if socket
        .send(close_msg(
            close_code::NORMAL,
            "Usuario successfully added".into(),
        ))
        .await
        .is_err()
    {
        // Log in case of error
        tracing::error!("Error sending sucessfully added response to client");
    };
}

// Enroll the print connecting to the Print server socket
// See fingerprint-rs crate for more info
pub(crate) async fn enroll_print(ws: &mut WebSocket) -> Result<String, String> {
    // Set the body with the proper action
    let body = Body {
        action: Action::Enroll,
        paths: vec![],
    };
    // Serialize the body to JSON
    let json_body = serde_json::json!(body).to_string();

    // Get the socket ip and port
    let sock_ip = crate::SOCKET_IP.to_string();
    let sock_port = crate::SOCKET_PORT.to_string();

    // Connect to the socket
    let (mut socket, _response) =
        tungstenite::connect(format!("ws://{}:{}/socket", sock_ip, sock_port))
            .map_err(|e| e.to_string())?;
    // Send JSON to the socket to start enroll action
    socket
        .send(tungstenite::Message::text(json_body.clone()))
        .map_err(|e| e.to_string())?;

    // Declare a path variable
    let mut path = String::default();
    // Loop until the fingerprint finish to enroll the new print and returned the new print path
    loop {
        let msg = match socket.read() {
            Ok(msg) => msg,
            Err(e) => match e {
                tungstenite::Error::ConnectionClosed => {
                    break;
                }
                _ => {
                    return Err(e.to_string());
                }
            },
        };

        if let tungstenite::Message::Text(msg) = msg {
            if let Ok(success) = serde_json::from_str::<Sucess>(&msg) {
                if success.result {
                    path = success.path.unwrap().to_string();
                } else {
                    return Err("Error saving print to file".into());
                }
                break;
            } else {
                let ws_msg = ws::Message::Text(msg.clone());
                ws.send(ws_msg).await.map_err(|e| e.to_string())?;
            };
        }
    }

    // Return the new print path
    tracing::info!("Enroll path: {}", &path);
    path = path.trim_matches(char::from(0)).to_string();
    dbg!(&path);
    Ok(path)
}

// Close message with a custom close code and reason
pub(crate) fn close_msg(close_code: u16, reason: String) -> ws::Message {
    ws::Message::Close(Some(CloseFrame {
        code: close_code,
        reason: Cow::Owned(reason),
    }))
}

// Send and close the socket
pub(crate) async fn send_and_close(err: (u16, String), mut socket: WebSocket) {
    let (error_code, error_message) = err;
    tracing::info!("{}", error_message);
    if let Err(send_error) = socket.send(close_msg(error_code, error_message)).await {
        println!("Failed to send close message: {:?}", send_error);
    }
}

// Check if the data provided by the client is a valid NewPerson struct
// and that the new rut is valid and is not already registered
pub(crate) async fn validate_message(
    data: String,
    pool: Arc<Pool>,
) -> Result<NewPersona, (u16, String)> {
    let persona: NewPersona = match serde_json::from_str(&data) {
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

use super::enroll::{
    close_msg, enroll_print, send_and_close, DB_INSERTION_ERR, FP_SENSOR_ERR, INVALID_RUT_ERR,
};
use crate::{
    api::utils::{internal_error, is_valid_num_rut},
    database::{
        entities::personas::{self, Entity as Personas},
        pool::Pool,
    },
};
use axum::{
    extract::{
        ws::{self, close_code, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use std::sync::Arc;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
struct Rut {
    rut: String,
}

// Handle the new connection and upgrade to a WebSocket
pub async fn reroll_persona(ws: WebSocketUpgrade, State(pool): State<Arc<Pool>>) -> Response {
    ws.on_failed_upgrade(|err| {
        tracing::error!("Failed to upgrade connection: {}", err);
    })
    .on_upgrade(move |socket| handle_socket(socket, pool.clone()))
}

// Handle a new socket connection from the client
async fn handle_socket(mut socket: WebSocket, pool: Arc<Pool>) {
    // Get the first message from the client, this should be a JSON with the new persona data
    let data = match socket.recv().await {
        Some(Ok(ws::Message::Text(data))) => data,
        _ => return,
    };

    // Validate that the message is a valid JSON of a new persona
    // check if the rut and email provided are valid a don't exist already
    let rut: Rut = match serde_json::from_str(&data) {
        Ok(rut) => rut,
        Err(_) => {
            send_and_close((INVALID_RUT_ERR, "".to_string()), socket).await;
            return;
        }
    };

    if !is_valid_num_rut(&rut.rut) {
        send_and_close((INVALID_RUT_ERR, "".to_string()), socket).await;
        return;
    }

    let querie = Personas::find_by_id(&rut.rut).filter(personas::Column::IsDisabled.eq(false));
    let persona_row = match querie.one(pool.get_db()).await.map_err(internal_error) {
        Ok(persona_row) => persona_row,
        Err(_) => {
            send_and_close((INVALID_RUT_ERR, "".to_string()), socket).await;
            return;
        }
    };

    if let Some(persona_row) = persona_row {
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
           // Todo: Update the fingerprint path on database

        let mut persona = persona_row.into_active_model();
        persona.print_path = Set(print);

        let _ = match Personas::update_many()
            .set(persona)
            .filter(personas::Column::Rut.eq(&rut.rut))
            .exec(pool.get_db())
            .await
        {
            Ok(a) => {
                tracing::info!("Updated persona: {:?}", a);
                a
            }
            Err(err) => {
                tracing::error!("Error updating persona: {:?}", err);
                send_and_close((DB_INSERTION_ERR, "".to_string()), socket).await;
                return;
            }
        };
    }

    // Close the socket normally
    if socket
        .send(close_msg(
            close_code::NORMAL,
            "Usuario successfully updated".into(),
        ))
        .await
        .is_err()
    {
        // Log in case of error
        tracing::error!("Error sending sucessfully added response to client");
    };
}

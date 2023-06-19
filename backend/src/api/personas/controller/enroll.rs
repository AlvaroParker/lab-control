use super::utils::{Action, Body, NewPersona, Outer};
use crate::{
    api::{
        personas::controller::utils::rut_exists_or_email,
        utils::{internal_error, is_valid_num_rut},
    },
    database::entities::personas,
    database::pool::Pool,
};
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::ActiveModelTrait;
use serde_json::json;
use std::{
    io::{self, ErrorKind, Read, Write},
    net::TcpStream,
    sync::Arc,
    time::Duration,
};

pub async fn enroll_persona(
    State(pool): State<Arc<Pool>>,
    Json(persona): Json<NewPersona>,
) -> Result<String, (StatusCode, String)> {
    if !is_valid_num_rut(&persona.rut) {
        return Err((StatusCode::BAD_REQUEST, "Invalid rut".into()));
    }
    // Check if the rut or the email is already in the database
    // It might be easier to make the ADD querie directly and handle the error
    // Todo: handle the database error instead of doing 2 queries
    if rut_exists_or_email(pool.clone(), &persona.rut, &persona.correo_uai).await? {
        return Err((StatusCode::CONFLICT, "Rut or email already exists".into()));
    }
    // Get print path, clean path, check if it's valid, return error if not
    let path = enroll_print().map_err(internal_error)?;
    let path = match path.split_whitespace().collect::<Vec<&str>>().get(1) {
        Some(s) => s.to_string(),
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            ));
        }
    };

    // Generate the new persona row
    let json_persona = json!(Outer::new(persona, path));
    dbg!(&json_persona.to_string());
    let persona_model = personas::ActiveModel::from_json(json_persona).map_err(internal_error)?;

    // Insert the new row
    persona_model
        .insert(pool.get_db())
        .await
        .map_err(internal_error)?;
    Ok("Person successfully added".into())
}
fn enroll_print() -> Result<String, io::Error> {
    let body = Body {
        action: Action::Enroll,
        paths: vec![],
    };
    let json_body = json!(body).to_string();
    let sock_ip = std::env::var("SOCKET_IP").unwrap_or("127.0.0.1".into());
    let sock_port = std::env::var("SOCKET_PORT").unwrap_or("5000".into());
    let mut stream = TcpStream::connect_timeout(
        &format!("{}:{}", sock_ip, sock_port)
            .parse()
            .map_err(|e| io::Error::new(ErrorKind::Other, e))?,
        Duration::from_secs(3),
    )?;

    // Write a message to the server.
    stream.write(json_body.as_bytes())?;

    stream.shutdown(std::net::Shutdown::Write)?;
    let mut path = String::new();
    loop {
        // Read a response from the server.
        let mut buf = [0; 1024];
        let n = stream.read(&mut buf)?;

        // If the server has closed the connection, break out of the loop.
        if n == 0 {
            break;
        }

        // Print the response.
        path = String::from_utf8(buf.to_vec())
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?;
        // Print the response.
    }
    path = path.trim_matches(char::from(0)).to_string();
    dbg!(&path);
    Ok(path)
}

use std::{cell::RefMut, net::TcpStream};

use tungstenite::{Message, WebSocket};

// Receives a TcpStream (the client) and a message to send to that client
pub fn send_message(mut address: RefMut<WebSocket<TcpStream>>, message: &str) {
    let msg = Message::Text(message.into());
    match address.write_message(msg) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{}", e);
            eprintln!("Message: {:?}", message);
        }
    };
}

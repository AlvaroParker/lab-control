use std::{cell::RefMut, io::Write, net::TcpStream};

// Receives a TcpStream (the client) and a message to send to that client
pub fn send_message(mut address: RefMut<TcpStream>, message: &str) {
    address.write_all(message.as_bytes()).unwrap();
}

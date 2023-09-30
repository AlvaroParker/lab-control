use std::cell::RefCell;
use std::error::Error;
use std::net::{IpAddr, SocketAddrV4, TcpListener};
use std::rc::Rc;

use libfprint_rs::FpContext;
use local_ip_address::linux::local_ip;
use serde::{Deserialize, Serialize};
use tungstenite::protocol::frame::coding::CloseCode;
use tungstenite::protocol::CloseFrame;
use tungstenite::{accept, Message};

use crate::enroll::run_enrollment;
use crate::utilities::get_device;
use crate::verify::run_verification;

/// Run the socket server
/// This will create a socket server and listen for request, in case
/// an `enroll` or `verify` request is received, the proper function
/// will be executed

#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
    pub action: Action,
    pub paths: Vec<String>,
}

// Serialize and deserialize the struct to JSON.
// Possible values for Actions are verify and enroll
#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    #[serde(alias = "verify")]
    Verify,
    #[serde(alias = "enroll")]
    Enroll,
}

pub fn server() -> Result<(), Box<dyn Error>> {
    // Get the local ip (i.e 192.168.68.34)
    let ip = local_ip().expect("Couldn't get local ip");
    // let ip = IpAddr::from_str("127.0.0.1").unwrap();
    // Check if the ip is an Ipv4
    let addr = match ip {
        IpAddr::V4(addr) => addr,
        _ => {
            eprintln!("Invalid address");
            std::process::exit(1);
        }
    };
    // Create the socket address with the given port
    let sock_addr = SocketAddrV4::new(addr, 5000);
    println!("Binding to {}", sock_addr);

    // Bind that address
    let listener = TcpListener::bind(sock_addr).unwrap();
    // Create a FpContext
    let ctx = FpContext::new();
    // Get the devices. Possible errors: No devices attached,
    // permissions errors, device not available
    let dev = match get_device(&ctx) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    // Check permissions
    dev.open_sync(None)?;
    // For each clients that connects to the socket:
    for stream in listener.incoming() {
        let websocket = match accept(stream?) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{:?}", e);
                std::process::exit(1)
            }
        };

        let stream = Rc::new(RefCell::new(websocket));

        // stream.borrow_mut().read(&mut buff[..]).unwrap();
        let msg = stream.borrow_mut().read_message().unwrap();
        if let Message::Text(msg) = msg {
            let body: Body = serde_json::from_str(&msg).unwrap();
            match body.action {
                Action::Verify => {
                    match run_verification(stream.clone(), &dev, body.paths) {
                        Ok(()) => {}
                        Err(err) => {
                            // Set the error message
                            let error_msg = format!("Error while running verification: {}", err);
                            // Close the socket with error
                            stream.borrow_mut().close(Some(CloseFrame {
                                code: CloseCode::Error,
                                reason: error_msg.into(),
                            }))?;
                            eprintln!("Error while running verification: {}", err);
                        }
                    };
                }
                Action::Enroll => {
                    match run_enrollment(stream, &dev) {
                        Ok(()) => {}
                        Err(err) => {
                            eprintln!("Error while running enrollment: {}", err);
                        }
                    };
                }
            }
        }
        // If the clients request an enroll action:
    }
    Ok(())
}


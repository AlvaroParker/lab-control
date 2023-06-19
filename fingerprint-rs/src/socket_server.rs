use std::cell::RefCell;
use std::error::Error;
use std::io::Read;
use std::net::{IpAddr, SocketAddrV4, TcpListener};
use std::rc::Rc;

use libfprint_rs::context::FpContext;
use libfprint_rs::error::GError;
use local_ip_address::linux::local_ip;
use serde::{Deserialize, Serialize};

use crate::enroll::run_enrollment;
use crate::utilities::get_device;
use crate::verify::run_verification;

/// Run the socket server
/// This will creaet a socket server and listen for request, in case
/// an `enroll` or `verify` request is received, the proper function
/// will be executed

#[derive(Serialize, Deserialize, Debug)]
pub struct Body {
    pub action: Action,
    pub paths: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    #[serde(alias = "verify")]
    Verify,
    #[serde(alias = "enroll")]
    Enroll,
}

pub fn server() -> Result<(), impl Error> {
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
    dev.open()?;
    // For each clients that connects to the socket:
    for stream in listener.incoming() {
        let stream = Rc::new(RefCell::new(stream.unwrap()));
        // stream.borrow_mut().read(&mut buff[..]).unwrap();
        let mut msg = String::new();
        stream.borrow_mut().read_to_string(&mut msg).unwrap();
        // If the clients request an enroll action:
        let body: Body = serde_json::from_str(&msg).unwrap();
        match body.action {
            Action::Verify => {
                run_verification(stream, &dev, body.paths)?;
            }
            Action::Enroll => {
                run_enrollment(stream, &dev)?;
            }
        }
    }
    Ok::<(), GError>(())
}

use std::{cell::RefCell, env, error::Error, net::TcpStream, rc::Rc};

use crate::{socket::send_message, write_to_file::write_to_file};
use libfprint_rs::{FpDevice, FpPrint, GError};
use serde::Serialize;
use tungstenite::WebSocket;
use uuid::Uuid;

#[derive(Serialize)]
struct ProcessMsg {
    total: i32,
    current: i32,
    error: bool,
}

#[derive(Serialize)]
struct Sucess<'a> {
    result: bool,
    path: Option<&'a str>,
}

// Run enrollment
pub fn run_enrollment(
    addr: Rc<RefCell<WebSocket<TcpStream>>>,
    dev: &FpDevice,
) -> Result<(), Box<dyn Error>> {
    // Check if enroll returned an error, return with error if it did
    let print = match enroll(Rc::clone(&addr), dev) {
        Ok(p) => p,
        Err(e) => return Err(e),
    };
    // Write to file, return error in case of one
    let name = Uuid::new_v4().to_string();
    match write_to_file(&print, &name) {
        Ok(_) => {
            if let Ok(dir) = env::current_dir() {
                let path = format!("{}/fingerprints/{}", dir.to_str().unwrap(), name);
                let success = Sucess {
                    result: true,
                    path: Some(&path),
                };
                let message = serde_json::to_string(&success).unwrap();
                send_message(addr.borrow_mut(), &message);
            }
        }
        Err(e) => {
            let success = Sucess {
                result: false,
                path: None,
            };
            let message = serde_json::to_string(&success).unwrap();
            send_message(addr.borrow_mut(), &message);
            eprintln!("{}", e);
        }
    };
    _ = addr.borrow_mut().close(None);
    // Ok = everything went well
    Ok(())
}

// The actual enroll function
fn enroll(
    sock_addr: Rc<RefCell<WebSocket<TcpStream>>>,
    dev: &FpDevice,
) -> Result<FpPrint, Box<dyn Error>> {
    // Check if the devices is open, if not, open it
    if !dev.is_open() {
        dev.open_sync(None)?;
    }

    // Create a template print
    let template_print = FpPrint::new(&dev);
    // Enroll
    let print = dev.enroll_sync(
        template_print,
        None,
        Some(callback_fn),
        Some(sock_addr.clone()),
    )?;
    // Close the device
    dev.close_sync(None)?;
    // Return the device
    Ok(print)
}

// Callback to call each time the user finger interacts
// with the sensor
fn callback_fn(
    device: &FpDevice,
    completed_stages: i32,
    _print: Option<FpPrint>,
    error: Option<GError>,
    user_data: &Option<Rc<RefCell<WebSocket<TcpStream>>>>,
) {
    // If there's no error and if the user data was successfully delivered to the funciton:
    if error.is_none() && user_data.is_some() {
        let data = user_data.as_ref().unwrap();
        let d = data.borrow_mut();
        let msg = ProcessMsg {
            current: completed_stages,
            total: device.nr_enroll_stage(),
            error: false,
        };
        let message = serde_json::to_string(&msg).unwrap();
        send_message(d, message.as_str());
    } else if error.is_some() {
        let data = user_data.as_ref().unwrap();
        let d = data.borrow_mut();
        let msg = ProcessMsg {
            current: completed_stages,
            total: device.nr_enroll_stage(),
            error: true,
        };
        let message = serde_json::to_string(&msg).unwrap();
        send_message(d, &message);
        // If there's an error, send it to the socket client
        eprintln!(
            "Enroll stage {} of {} failed with error {}",
            completed_stages,
            device.nr_enroll_stage(),
            error.unwrap()
        );
    }
}

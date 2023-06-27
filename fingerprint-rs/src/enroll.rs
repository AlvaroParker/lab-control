use std::{cell::RefCell, env, net::TcpStream, rc::Rc};

use crate::{socket::send_message, write_to_file::write_to_file};
use libfprint_rs::{device::FpDevice, error::GError, print::FpPrint};
use tungstenite::WebSocket;
use uuid::Uuid;

// Run enrollment
pub fn run_enrollment(
    addr: Rc<RefCell<WebSocket<TcpStream>>>,
    dev: &FpDevice,
) -> Result<(), GError<'static>> {
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
                let mesage = format!("SUCCESS {}/fingerprints/{}", dir.to_str().unwrap(), name);
                send_message(addr.borrow_mut(), &mesage);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
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
) -> Result<FpPrint<'static>, GError<'static>> {
    // Check if the devices is open, if not, open it
    if !dev.is_open() {
        dev.open()?;
    }

    // Create a template print
    let template_print = FpPrint::new(&dev);
    // Enroll
    let print = dev.enroll(template_print, Some(callback_fn), Some(sock_addr.clone()))?;
    // Close the device
    dev.close()?;
    // Return the device
    Ok(print)
}

// Callback to call each time the user finger interacts
// with the sensor
fn callback_fn(
    device: &FpDevice,
    completed_stages: i32,
    _print: FpPrint,
    error: Option<GError>,
    user_data: &mut Option<Rc<RefCell<WebSocket<TcpStream>>>>,
) {
    // If there's no error and if the user data was successfully delivered to the funciton:
    if error.is_none() && user_data.is_some() {
        let data = user_data.as_ref().unwrap();
        let d = data.borrow_mut();
        let message = format!("{} of {}", completed_stages, device.get_nr_enroll_stages());
        send_message(d, message.as_str());
    } else if error.is_some() {
        // If there's an error, send it to the socket client
        eprintln!(
            "Enroll stage {} of {} failed with error {}",
            completed_stages,
            device.get_nr_enroll_stages(),
            error.unwrap()
        );
    }
}

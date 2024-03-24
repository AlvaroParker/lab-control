use std::{cell::RefCell, error::Error, fs, io::Read, net::TcpStream, rc::Rc};

use libfprint_rs::{FpDevice, FpPrint, GError};
use tungstenite::WebSocket;

use crate::socket::send_message;

// Collect fingerprints from the
fn collect_prints(paths: Vec<String>) -> Vec<FpPrint> {
    let mut prints = Vec::new();

    for path in paths {
        let file = fs::File::open(&path);
        let mut buff = Vec::new();

        match file {
            Ok(mut file) => {
                match file.read_to_end(&mut buff) {
                    Ok(_r) => {
                        let print = FpPrint::deserialize(buff.as_slice()).unwrap();
                        print.set_username(&path);
                        prints.push(print);
                    }
                    Err(_) => {}
                };
            }
            Err(_e) => {}
        }
    }
    prints
}

// Run the verification, this takes a socket client stream to report match/not match
// and the finger sensor device
pub fn run_verification(
    addr: Rc<RefCell<WebSocket<TcpStream>>>,
    dev: &FpDevice,
    paths: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    if !dev.is_open() {
        dev.open_sync(None)?;
    }

    let prints = collect_prints(paths);

    let mut new_print = FpPrint::new(&dev);
    let _matched_print = dev.identify_sync(
        &prints,
        None,
        Some(callback_function),
        Some(addr.clone()),
        Some(&mut new_print),
    )?;
    dev.close_sync(None)?;
    _ = addr.borrow_mut().close(None);
    Ok(())
}

// Callback function to report if there's a match, or if at the end of
// the array, a matching fingerprint was not found
fn callback_function(
    _device: &FpDevice,
    matched_print: Option<FpPrint>, // The matched print, if any.
    _new_print: FpPrint,            // New print scanned.
    _error: Option<GError>,         // Optinal error in case of an error.
    match_data: &Option<Rc<RefCell<WebSocket<TcpStream>>>>, // User data can be any data type.
) {
    if let Some(print) = matched_print {
        let data = match_data.as_ref().unwrap();
        let d = data.borrow_mut();
        if let Some(path) = print.username() {
            let msg = format!("verification success {}", path);
            send_message(d, &msg);
        }
    } else {
        let data = match_data.as_ref().unwrap();
        let d = data.borrow_mut();
        send_message(d, "verification error");
    }
}

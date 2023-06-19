use socket_server::server;

mod enroll;
mod socket;
mod socket_server;
mod utilities;
mod verify;
mod write_to_file;

fn main() {
    // Start socket, print error if any and return
    match server() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
        }
    };
}

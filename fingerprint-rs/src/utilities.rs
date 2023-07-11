use libfprint_rs::{FpContext, FpDevice};
use std::{
    error::Error,
    io::{self, ErrorKind},
};

// Get the fingerprint devices attached to the host
pub fn get_device(ctx: &FpContext) -> Result<FpDevice<'_>, impl Error> {
    let devices = ctx.get_devices();
    let dev = match devices.iter().next() {
        Some(d) => d,
        None => {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                "ERROR: No fingerprint device found.",
            ))
        }
    };

    Ok(dev)
}

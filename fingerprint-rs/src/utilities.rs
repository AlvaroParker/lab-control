use libfprint_rs::{FpContext, FpDevice};
use std::{
    error::Error,
    io::{self, ErrorKind},
};

// Get the fingerprint devices attached to the host
pub fn get_device(ctx: &FpContext) -> Result<FpDevice, Box<dyn Error>> {
    let devices = ctx.devices();
    if let Some(dev) = devices.get(0).cloned() {
        return Ok(dev);
    } else {
        return Err(Box::new(io::Error::new(
            ErrorKind::Other,
            "No devices found",
        )));
    }
}

use libfprint_rs::print::FpPrint;
use std::{
    fs,
    io::{self, Write},
};

// Takes a print, serializes it and saves that data to a file
pub fn write_to_file(print: &FpPrint, name: &String) -> Result<(), io::Error> {
    fs::create_dir_all("fingerprints")?;
    let file_name = format!("fingerprints/{}", name);
    let mut file = fs::File::create(file_name.as_str())?;
    let print_data = print.serialize().unwrap();
    let buff = print_data.as_slice();
    file.write_all(buff)?;
    Ok(())
}

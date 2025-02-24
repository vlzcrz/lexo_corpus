use std::{
    fs::File,
    io::{Error, Write},
};

use jiff::{Unit, Zoned};

pub fn create_log_instance() -> Result<File, jiff::Error> {
    let now = Zoned::now().datetime().round(Unit::Second)?;
    let mut file = File::create(format!("./logs/{}.txt", now)).unwrap();
    write_log_result(format!("Initialized log process at: {}", now), &mut file).unwrap();
    Ok(file)
}

pub fn write_log_result(msg: String, file: &mut File) -> Result<(), Error> {
    file.write(msg.as_bytes())?;
    Ok(())
}

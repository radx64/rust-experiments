use std::io::{self, Read};
use std::fs::{self, File};

pub fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err (e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(String::from(username.trim())),
        Err(e) => Err(e),
    }
}

pub fn read_username_from_file_questionmark_op() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(String::from(username.trim()))
}

pub fn read_username_from_file_fs() -> Result<String, io::Error> {
    Ok(String::from(fs::read_to_string("hello.txt")?.trim()))
}

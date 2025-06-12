use std::fs::File;
use std::io::ErrorKind;

mod username;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    match username::read_username_from_file() {
        Ok(name) => println!("Hello {name}!"),
        Err(e) => println!("Error {e:?}"),
    }

    match username::read_username_from_file_questionmark_op() {
        Ok(name) => println!("Hello {name}!"),
        Err(e) => println!("Error {e:?}"),       
    }

    match username::read_username_from_file_fs() {
        Ok(name) => println!("Hello {name}!"),
        Err(e) => println!("Error {e:?}"),       
    }

}

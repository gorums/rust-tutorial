use std::fs::File;
use std::io::{self, Read};

fn main() {
    let username = read_username_from_file();
    let username2 = read_username_from_file2();    

    match username {
        Ok(name) => println!("Username 1: {}", name),
        Err(e) => println!("Error reading username 1: {}", e),
    }

    match username2 {
        Ok(name) => println!("Username 2: {}", name),
        Err(e) => println!("Error reading username 2: {}", e),
    }

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
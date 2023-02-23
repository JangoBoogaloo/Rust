use std::fs::{File, remove_file};
use std::io::{self, Read};

fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
    let mut username = String::new();

    File::open(file_name)?.read_to_string(&mut username)?;

    Ok(username)
}

fn main() {
    let file_name = "Hellow.txt";
    File::create(file_name).expect("Fail to create file");
    let username = read_username_from_file(file_name).expect("File does not exist");
    remove_file(file_name).expect("Fail to delete file");
    print!("UserName: {username}");
}
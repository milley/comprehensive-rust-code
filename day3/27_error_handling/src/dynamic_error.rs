use std::{error::Error, fs::File, io::Read};
use thiserror::Error;

#[derive(Debug, Clone, Eq, Error, PartialEq)]
#[error("Found no username in {0}")]
struct EmptyUsesrnameError(String);

fn read_username(path: &str) -> Result<String, Box<dyn Error>> {
    let mut username = String::with_capacity(100);
    File::open(path)?.read_to_string(&mut username)?;
    if username.is_empty() {
        return Err(EmptyUsesrnameError(String::from(path)).into());
    }
    Ok(username)
}

fn main() {
    match read_username("config.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err) => println!("Error: {}", err),
    }
}

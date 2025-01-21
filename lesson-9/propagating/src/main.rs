use std::fs::File;
use std::io::{self, Read};

fn read_name () -> Result<String, io::Error> {
    let mut file = File::open("name.txt")?;
    let mut name = String::new();
    file.read_to_string(&mut name)?;
    Ok(name)
}

fn main() {
    let name = read_name();
    match name {
        Ok(name) => println!("Hello, {name}!"),
        Err(error) => println!("Error: {error}"),
    }
}

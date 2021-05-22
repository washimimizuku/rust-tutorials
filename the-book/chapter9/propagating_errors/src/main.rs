use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;

// Without using a function, we would have to change main result.
/*
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
*/

fn main() {
    let username = read_username_from_file();

    match username {
        Ok(username) => println!("Hello {}", username),
        Err(error) => panic!("Usename not found: {:?}", error),
    };

    let username = read_username_from_file_simpler();

    match username {
        Ok(username) => println!("Hello {}", username),
        Err(error) => panic!("Usename not found: {:?}", error),
    };

    let username = read_username_from_file_shorter();

    match username {
        Ok(username) => println!("Hello {}", username),
        Err(error) => panic!("Usename not found: {:?}", error),
    };

    let username = read_username_from_file_shortest();

    match username {
        Ok(username) => println!("Hello {}", username),
        Err(error) => panic!("Usename not found: {:?}", error),
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => return Err(error),
    }
}

// With ? operator
fn read_username_from_file_simpler() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}


fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
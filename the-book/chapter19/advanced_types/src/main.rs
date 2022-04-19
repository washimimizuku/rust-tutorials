use std::fmt;
use std::io::Error;

// Creating Type Synonyms with Type Aliases
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}

type ResultImproved<T> = std::result::Result<T, std::io::Error>;

pub trait WriteImproved {
    fn write(&mut self, buf: &[u8]) -> ResultImproved<usize>;
    fn flush(&mut self) -> ResultImproved<()>;

    fn write_all(&mut self, buf: &[u8]) -> ResultImproved<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> ResultImproved<()>;
}

fn main() {
    // Creating Type Synonyms with Type Aliases
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --snip--
    }

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        // --snip--
        Box::new(|| ())
    }
    
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn better_takes_long_type(f: Thunk) {
        // --snip--
    }

    fn better_returns_long_type() -> Thunk {
        // --snip--
        Box::new(|| ())
    }
}

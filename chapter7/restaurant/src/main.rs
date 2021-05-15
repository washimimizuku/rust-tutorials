// Idiomatic way of imporing for structs
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("HashMap: {:#?}", map);

    // External package
    use rand::Rng;
    
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The random number is: {}", secret_number);

    // Nested paths
    use std::{cmp::Ordering, io};
    // Same has:
    // use std::cmp::Ordering;
    // use std::io;

    use std::os::{self, raw};
    // Same has:
    // use std::os;
    // use std::os::raw;

    // Glob operator
    use std::net::*;
}

// Bringing two items with the same name into scope
use std::fmt;
use std::io;

// fn function1() -> fmt::Result {}
// fn function2() -> io::Result {}

// With as
use std::fmt::Result;
use std::io::Result as IoResult;

// fn function1() -> Result {}
// fn function2() -> IoResult {}

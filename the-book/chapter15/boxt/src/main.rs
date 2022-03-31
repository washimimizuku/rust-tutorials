use crate::List::{Cons, Nil};

fn main() {
    // Simple use of Box<T>
    let b = Box::new(5);
    println!("b = {}", b);

    // Use of Box<T> for recursive types
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

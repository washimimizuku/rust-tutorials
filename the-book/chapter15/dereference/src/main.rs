use std::ops::Deref;

fn main() {
    // Using dereference
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Using Box<T>
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Using MyBox<T>
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Implicit Deref
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // If there was no implicit Deref
    hello(&(*m)[..]);
}

// Defining our own smart pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Implicit Deref
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
fn main() {
    // Matching Literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Named Variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Go 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Matched, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    // Multiple Patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

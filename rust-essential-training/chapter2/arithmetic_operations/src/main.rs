fn main() {
    let a = 10;
    let b = 3;

    // Adition
    let c = a + b;
    println!("c is {}", c);

    // Subtraction
    let d = a - b;
    println!("d is {}", d);

    // Multiplication
    let e = a * b;
    println!("e is {}", e);

    // Division of integers returns an integer
    let f = a / b;
    println!("f is {}", f);

    // Remainer
    let g = a % b;
    println!("g is {}", g);

    // Division of floats
    let x = 10.0;
    let y = 3.0;

    let z = x / y;
    println!("z is {}", z);

    // Mix numeric types
    let x2 = 10;
    let y2 = 3.0;

    let z2 = x2 as f64 / (y2 + 1.0);
    println!("z2 is {}", z2);
}

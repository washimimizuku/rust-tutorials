fn main() {
    let a = true;
    let b = false;

    // Bitwise operations on booleans
    println!("a is {} and b is {}", a, b);
    println!("NOT a is {}", !a);
    println!("a AND B is {}", a & b);
    println!("a OR B is {}", a | b);
    println!("a XOR B is {}", a ^ b);

    let c = (a ^ b) | (a & b);

    println!("c is {}", c);

    // Short-circuiting OR (only executes right side if necessary)
    let d = (a ^ b) || (a & b);

    println!("d is {}", d);

    // Short-circuiting AND (only executes right side if necessary)
    let e = (a & b) && (a ^ b);

    println!("e is {}", e);
}

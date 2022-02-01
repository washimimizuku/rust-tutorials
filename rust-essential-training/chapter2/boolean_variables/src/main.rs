fn main() {
    let a = true;
    let b = false;

    println!("a is {} and b is {}", a, b);
    println!("NOT a is {}", !a);
    println!("a AND B is {}", a & b);
    println!("a OR B is {}", a | b);
    println!("a XOR B is {}", a ^ b);

    let c = (a ^ b) | (a & b);
    
    println!("c is {}", c);
}

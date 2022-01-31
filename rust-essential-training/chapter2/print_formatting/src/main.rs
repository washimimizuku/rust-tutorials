fn main() {
    let x = 10.0;
    let y = 3.0;
    let z = x / y;

    println!("z is {}", z);
    println!("z is {:.3}", z); // Decimal places to show = 3
    println!("z is {:8.3}", z); // Space occupied by number = 8
    println!("z is {:08.3}", z); // Fill space on left with 0
    println!("z is {:08.3}\nx is {}", z, x); // Two variables separated by new line
    print!("z is {:08.3}\nx is {}\n", z, x); // Do not print new line at the end by default
    println!("z is {0:08.3}\nx is {1}\nonce again, z is {0}", z, x); // Two variables separated by new line
}

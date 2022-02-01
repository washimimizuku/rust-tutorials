fn main() {
    let mut value = 0b1111_0101u8;

    println!("value is {}", value);
    println!("value is {:08b}", value);

    // NOT bitwise operator
    value = !value;
    
    println!("value is {:08b}", value);

    // AND bitwise operator
    value = value & 0b1111_0111;
    
    println!("value is {:08b}", value);
    println!("bit 6 is {}", value & 0b0100_0000);
    println!("bit 2 is {}", value & 0b0000_0010);

    // OR bitwise operator
    value = value | 0b0100_0000;
    
    println!("value is {:08b}", value);

    // XOR bitwise operator
    value = value ^ 0b0101_0101;
    
    println!("value is {:08b}", value);

    // Left Shift bitwise operator
    value = value << 4;
    
    println!("value is {:08b}", value);

    // Right Shift bitwise operator
    value = value >> 2;

    println!("value is {:08b}", value);
}

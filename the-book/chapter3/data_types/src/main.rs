fn main() {
    // Integer Types:
    // ---------------------------
    // Length  | Unsigned | Signed
    // ---------------------------
    // 8-bit   | u8       | i8
    // 16-bit  | u16      | i16
    // 32-bit  | u32      | i32 (default)
    // 64-bit  | u64      | i64
    // 128-bit | u128     | i128
    // arch    | usize    | isize
    // ---------------------------
    
    let decimal = 98_200; // Number literals can also use _ as a visual separator
    println!("The value of decimal is {}", decimal);

    let hex = 0xff;
    println!("The value of hex is {}", hex);

    let octal = 0o77;
    println!("The value of octal is {}", octal);

    let binary = 0b1111_0000;
    println!("The value of binary is {}", binary);

    let byte: u8 = b'A';
    println!("The value of byte is {}", byte);

    // Float Types:
    // --------------
    // Length  | Type
    // --------------
    // 32-bit  | f32
    // 64-bit  | f64 (default)
    // --------------

    let x = 2.0; // f64
    println!("The value of x is {}", x);

    let y: f32 = 3.0; // f32
    println!("The value of y is {}", y);

    // addition
    let sum = 5 + 10;
    println!("The value of sum is {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("The value of product is {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is {}", remainder);
    
    // Boolean Type
    let t = true;
    println!("The value of t is {}", t);

    let f: bool = false; // with explicit type annotation
    println!("The value of f is {}", f);

    // Character Type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("The value of c is {}", c);
    println!("The value of z is {}", z);
    println!("The value of heart_eyed_cat is {}", heart_eyed_cat);

    // Tuple Type (fixed length list)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The value of five_hundred is {}", five_hundred);
    println!("The value of six_point_four is {}", six_point_four);
    println!("The value of one is {}", one);

    // Array Type (variable length list)
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    println!("The value of months[6] is {}", months[6]);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a[0] is {}", a[0]);
    
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    println!("The value of a[0] is {}", a[0]);
}
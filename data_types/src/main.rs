fn main() {
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
    
    let t = true;
    println!("The value of t is {}", t);

    let f: bool = false; // with explicit type annotation
    println!("The value of f is {}", f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("The value of c is {}", c);
    println!("The value of z is {}", z);
    println!("The value of heart_eyed_cat is {}", heart_eyed_cat);

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

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a[0] is {}", a[0]);
    let a = [3; 5];
    println!("The value of a[0] is {}", a[0]);
}
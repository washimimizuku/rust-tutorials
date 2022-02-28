fn main() {
    println!("Hello, world!");

    another_function();

    yet_another_function(5, 6);

    print_labeled_measurement(5, 'h');

    let x = 10;

    let y = {
        let x = 3;
        x + 1
    };
    
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);

    let x = five();
    println!("The value of x is {}", x);

    let x = plus_one(5);
    println!("The value of x is {}", x);
}

fn another_function() {
    println!("A function.")
}

fn yet_another_function(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label)
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

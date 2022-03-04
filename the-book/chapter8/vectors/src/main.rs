fn main() {
    // Create vector
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("Vector v: {:?}", v);

    // Create vector with macro
    let v2 = vec![1, 2, 3, 4, 5];

    println!("Vector v2: {:?}", v2);

    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // This will give an error:
    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    // Thus will violate the ownership and bprrpwing rules
    /*
    let mut v3 = vec![1, 2, 3, 4, 5];

    let first = &v3[0];

    v3.push(6);

    println!("The first element is: {}", first);
    */

    // Iterate over the values in a Vector
    // Immutable
    let v4 = vec![100, 32, 57];
    println!("Vector v4: {:?}", v4);
    for i in &v4 {
        println!("{}", i)
    }

    // Mutable
    let mut v5 = vec![100, 32, 57];
    println!("Vector v5: {:?}", v5);
    for i in &mut v5 {
        *i += 50;
    }
    println!("Vector v5 after modification: {:?}", v5);

    // Use enum to store multiple types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("Row: {:?}", row);
}

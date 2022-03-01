fn main() {
    // Reference
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // Mutable reference
    let mut s = String::from("Hello");

    change(&mut s);

    println!("{}", s);

    // Multiple reference with mutable and non-mutable
    let mut s = String::from("Hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // Dangling reference
    // let reference_to_nothing = dangle();

    // Dangling references avoidance
    let s2 = no_dangle();
    println!("{}", s2);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world")
}

/*
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
*/

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

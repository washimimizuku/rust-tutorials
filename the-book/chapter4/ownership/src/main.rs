fn main() { // Scope starts
    let s = "hello"; // s is a literal -> goes to Stack
    println!("{}!", s);

    let s = String::from("hello"); // s is a String -> goes to Heap
    println!("{}!", s);

    let mut s = String::from("hello"); // s is a mutable String -> only possible on the Heap
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}!", s);

    // Copy

    let x = 5;
    let y = x; // Deep copy, two separate values
    println!("{}", x);
    println!("{}", y);

    let s1 = String::from("hello");
    let s2 = s1; // Move, only pointer is copied (not shallow copy because s1 becomes invalid)
    // println!("{}, world!", s1); -> Gives error[E0382]: borrow of moved value: `s1`
    println!("{}, world!", s2);

    // Clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    
    let x = 5;
    let y = x; // For literals of fixed size, acts as clone

    println!("x = {}, u = {}", x, y);
    
} // Scope ends -> All variables above will return their memory

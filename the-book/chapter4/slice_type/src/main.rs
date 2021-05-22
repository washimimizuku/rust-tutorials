fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    println!("{}", word);

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // Using slices
    let s2 = String::from("hello world");

    let hello = &s2[0..5]; // Or [..5]
    let world = &s2[6..11]; // Or [6..]
    let hello_world = &s2[..]; // Or [0..11]

    println!("{}", hello);
    println!("{}", world);
    println!("{}", hello_world);

    let s3 = String::from("hello world");

    let word = first_word_with_slices(&s3);

    println!("the first word is: {}", word);

    let my_string = String::from("hello world");

    // first_word_with_slices works on slices of Strings
    let word = first_word_with_slices(&my_string[..]);

    println!("the first word is: {}", word);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word_with_slices(&my_string_literal[..]);

    println!("the first word is: {}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_with_slices(&my_string_literal);

    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_with_slices(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}
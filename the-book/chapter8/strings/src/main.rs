fn main() {
    // Using collection functions
    let mut s = String::new();
    println!("Empty string: {:?}", s);
    s.push('t');
    s.push('e');
    s.push('s');
    s.push('t');
    println!("Filled string: {}", s);

    // With initial contents
    let data = "initial contents";
    let s2 = data.to_string();
    println!("String with initial content: {}", s2);
    
    // the method also works on a literal directly:
    let s3 = "alternative initial contents".to_string();
    println!("String with initial content: {}", s3);

    // Using from
    let s4 = String::from("alternative initial contents");
    println!("String using from: {}", s4);

    // Full UTF-8
    let hello = String::from("السلام عليكم");
    println!("String hello: {}", hello);
    let hello = String::from("Dobrý den");
    println!("String hello: {}", hello);
    let hello = String::from("Hello");
    println!("String hello: {}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("String hello: {}", hello);
    let hello = String::from("नमस्ते");
    println!("String hello: {}", hello);
    let hello = String::from("こんにちは");
    println!("String hello: {}", hello);
    let hello = String::from("안녕하세요");
    println!("String hello: {}", hello);
    let hello = String::from("你好");
    println!("String hello: {}", hello);
    let hello = String::from("Olá");
    println!("String hello: {}", hello);
    let hello = String::from("Здравствуйте");
    println!("String hello: {}", hello);
    let hello = String::from("Hola");
    println!("String hello: {}", hello);

    // Appending to a string
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    s1.push('!');
    println!("s1 is {}", s1);
    
    // Concatenation with the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 is {}", s3);

    let str1 = String::from("tic");
    let str2 = String::from("tac");
    let str3 = String::from("toe");

    let strx = str1 + "-" + &str2 + "-" + &str3;
    println!("strx is {}", strx);

    let str1 = String::from("tic");
    let str2 = String::from("tac");
    let str3 = String::from("toe");
    
    let strx = format!("{}-{}-{}", str1, str2, str3);
    println!("strx is {}", strx);

    // Slicing strings
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("hello slice is {}", s);

    // Iterating over Strings
    for c in "नमस्ते".chars() { // As chars
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() { // As bytes
        println!("{}", b);
    }
}

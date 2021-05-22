fn main() {
    // Lifetime representation
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+

    // Function with lifetime annotations
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // This respects lifetimes
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // This also respects lifetimes
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // This does not respect lifetimes
    println!("The longest string is {}", result);

    // Lifetimes with structs
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("The important excerpt is: {}", i.part);

    // Lifetime elisionfn
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
    println!("The first word is: {}", first_word("This is a test"));

    // Lifetime annotations in method definitions
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }

    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("The important excerpt is: {}", i.part);
    println!("The important excerpt level is: {}", i.level());
    println!("The important annoucement is: {}", i.announce_and_return_part("This is an announcement!!!"));

    // Static lifetime (global)
    let s: &'static str = "I have a static lifetime.";
    println!("This is static: {}", s);

    // Generic Type Parameters, Trait Bounds, and Lifetimes Together
    use std::fmt::Display;
    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where T: Display,
    {
        println!("Annoucement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest_with_an_announcement(string1.as_str(), string2.as_str(), "Important!!!");
    println!("The longest string is {}", result);
}

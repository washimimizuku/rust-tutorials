struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        // Using the Field Init Shorthand when Variables and Fields Have the Same Name
        email, // Same as "email: email,"
        username, // Same as "username: username,"
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };
    println!("User 1 username: {}", user1.username);
    println!("User 1 email: {}", user1.email);
    println!("User 1 sign_in_count: {}", user1.sign_in_count);
    println!("User 1 active: {}", user1.active);

    user1.email = String::from("anotheremail@example.com");
    println!("User 1 new email: {}", user1.email);

    let user2 = build_user(String::from("someone@example.com"),String::from("someusername456"));
    println!("User 2 email: {}", user2.email);

    // Creating Instances From Other Instances With Struct Update Syntax
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
        // Same as:
        // active: user1.active,
        // sign_in_count: user1.sign_in_count,
    };
    println!("User 3 username: {}", user3.username);
    println!("User 3 email: {}", user3.email);

    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    println!("Black: ({}, {}, {})", black.0, black.1, black.2);
    let origin = Point(0, 0, 0);
    println!("Origin: ({}, {}, {})", origin.0, origin.1, origin.2);
}

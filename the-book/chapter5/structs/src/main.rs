struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_short(email: String, username: String) -> User {
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
        email: String::from("someone1@example.com"),
        username: String::from("someusername1"),
        active: true,
        sign_in_count: 1
    };
    println!("User 1 username: {}", user1.username);
    println!("User 1 email: {}", user1.email);
    println!("User 1 sign_in_count: {}", user1.sign_in_count);
    println!("User 1 active: {}", user1.active);

    user1.email = String::from("anotheremail@example.com");
    println!("User 1 new email: {}", user1.email);

    let user2 = build_user(String::from("someone2@example.com"), String::from("someusername2"));
    println!("User 2 email: {}", user2.email);

    let user3 = build_user_short(String::from("someone3@example.com"), String::from("someusername3"));
    println!("User 3 email: {}", user3.email);

    // Creating Instances From Other Instances With Struct Update Syntax
    let user4 = User {
        email: String::from("someone4@example.com"),
        username: String::from("anotherusername4"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    println!("User 4 username: {}", user4.username);
    println!("User 4 email: {}", user4.email);

    let user5 = User {
        email: String::from("someone5@example.com"),
        ..user1
        // Same as:
        // username: user1.username,
        // active: user1.active,
        // sign_in_count: user1.sign_in_count,
    };
    println!("User 5 username: {}", user5.username);
    println!("User 5 email: {}", user5.email);

    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    println!("Black: ({}, {}, {})", black.0, black.1, black.2);
    let origin = Point(0, 0, 0);
    println!("Origin: ({}, {}, {})", origin.0, origin.1, origin.2);

    // Unit-Like Structs
    #[derive(Debug)]
    struct AlwaysEqual;

    let subject = AlwaysEqual;
    println!("subject is {:#?}", subject);
}

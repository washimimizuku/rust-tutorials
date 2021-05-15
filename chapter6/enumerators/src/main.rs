// Simple enum
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    // Struct that uses an enum
    #[derive(Debug)]
    struct IpAddrStruct {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    println!("home is: {:#?}", home);

    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
    println!("loopback is: {:#?}", loopback);

    // Enum with types
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String)
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home is: {:#?}", home);
    println!("loopback is: {:#?}", loopback);

    // Enum with many different types
    #[derive(Debug)]
    enum Message {
        Quit, // No data associated
        Move { x: i32, y: i32 }, // Anonymous struct
        Write(String), // String
        ChangeColor(i32, i32, i32) // three i32 values
    }

    impl Message {
        fn call(&self) {
            println!("Message called: {:#?}", self)
        }
    }

    let message1 = Message::Quit;
    message1.call();

    let message2 = Message::Move{ x: 1, y: 2 };
    message2.call();

    let message3 = Message::Write(String::from("hello"));
    message3.call();

    let message4 = Message::ChangeColor(255, 255, 255);
    message4.call();

    // Option Enum
    let some_number = Some(5);
    let some_string = Some("a string");
    println!("some_number is: {:#?}", some_number);
    println!("some_string is: {:#?}", some_string);

    let absent_number: Option<i32> = None;
    println!("absent_number is: {:#?}", absent_number);
}

fn route(ip_kind: IpAddrKind) {
    println!("Routing for {:#?}", ip_kind)
}
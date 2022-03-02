#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// Matching an enum
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// Matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    // Matching an enum
    let penny = Coin::Penny;
    println!("The value of a penny is {} cents.", value_in_cents(penny));
    let nickel = Coin::Nickel;
    println!("The value of a nickel is {} cents.", value_in_cents(nickel));
    let dime = Coin::Dime;
    println!("The value of a dime is {} cents.", value_in_cents(dime));
    let quarter = Coin::Quarter(UsState::Alaska);
    println!("The value of a quarter is {} cents.", value_in_cents(quarter));

    // Matching with Option<T>
    let five = Some(5);
    let six = plus_one(five);
    println!("The value of six is {:?}.", six);
    let none = plus_one(None);
    println!("The value of none is {:?}.", none);

    // The _ Placeholder
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        2 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("number not found"), // _ matches any other
    }

    let state = UsState::Alaska;
    match state {
        UsState::Alabama => println!("State is Alabama."),
        _ => println!("State not Alabama!"), // _ matches any other
    }

    // The variable placeholder
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}

fn add_fancy_hat() {
    println!("Added fancy hat.");
}

fn remove_fancy_hat() {
    println!("Removed fancy hat.");
}

fn move_player(num_spaces: u8) {
    println!("Walked {} steps.", num_spaces);
}
mod suppliers;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        #[allow(dead_code)]
        fn seat_at_table() {}
    }

    mod serving {
        #[allow(dead_code)]
        fn take_order() {}

        #[allow(dead_code)]
        fn serve_order() {}

        #[allow(dead_code)]
        fn take_payment() {}
    }
}

#[allow(dead_code)]
fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        #[allow(dead_code)]
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    #[allow(dead_code)]
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order()
    }

    fn cook_order() {}

    #[allow(dead_code)]
    fn order_supplies() {
        pub use crate::suppliers::ordering;
        ordering::order_products();
    }
}

use crate::front_of_house::hosting; // <-- Idiomatic way for functions
use self::front_of_house::hosting as self_hosting;
use crate::front_of_house::hosting::add_to_waitlist;

// Re-exporting function
pub use self::front_of_house::hosting as pub_hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // From use above with crate::...
    hosting::add_to_waitlist();

    // From use above with self::...
    self_hosting::add_to_waitlist();

    // From use above pointing to add_to_waitlist
    add_to_waitlist();

    // From re-exporting
    pub_hosting::add_to_waitlist();
    
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    #[allow(unused_variables)]
    let order1 = back_of_house::Appetizer::Soup;
    #[allow(unused_variables)]
    let order2 = back_of_house::Appetizer::Salad;
}
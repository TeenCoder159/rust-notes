#[allow(dead_code)]
#[allow(unused_imports)]
mod front_of_house {
    pub mod hosting {
        // pub keyword makes hosting module public to front_of_house
        pub fn add_to_waitlist() {}

        fn _seat_at_table() {}
    }
    mod serving {
        fn _take_order() {}

        fn _serve_order() {}

        fn _take_payment() {}
    }
}

fn _deliver_order() {}

mod back_of_house {
    fn _fix_incorrect_order() {
        _cook_order();
        super::_deliver_order();
        //super allows us to access functions from parent module
    }
    fn _cook_order() {}
    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String,
        // pub in structs makes only struct public not values inside
        // pub on values in struct make the value public
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("Peaches"),
            }
        }
    }
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
pub use crate::front_of_house::hosting;
// shortens the path to call functions
// requires pub as use brings the function into a new scope, making it private

pub fn eat_at_restaurant() {
    //absolute path to call add_to_waitlist function:
    crate::front_of_house::hosting::add_to_waitlist();
    // can shorten to:
    hosting::add_to_waitlist();
    // because of line 47

    /*----------Divider----------*/

    //relative path to call add_to_waitlist function:
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we like:
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal._seasonal_fruit = String::from("blueberries");

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}

// 2 ways to bring in same items with same name but different scope:
/*
use std::fmt;
use std::io;

    fn function1() -> fmt::Result {

    }
    fn function2() -> io::Result{

    }
*/
/*----------OR----------*/

//use std::fmt::Result as fmtResult;
//use std::io::Result as IoResult;
// then call fmtResult or IoResult respectively
// to bring 2 things into scope together:
//use std::{cmp::Ordering, io};
// or to bring 2 things like:
// std::io and std::io::Write we can do:
// use std::io{self, Write};
//to bring everything of a crate into scope, use * like this:
//use std::collections::*;

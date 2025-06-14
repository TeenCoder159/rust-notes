#![allow(dead_code)]
enum IpAddrKind {
    V4,
    V6,
}
// enum means that these are only possible values
// Basically a custom variable format (like i64 or u32)

/*
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr{
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
*/
// faster method for storing commented code above:
enum IpAddr {
    V4(String),
    V6(String),
}
enum NullOption<T> {
    None,
    Some(T),
}
// the <T> makes it so that the enum element with (T)...
// ...can have one value of any type

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColour(i32, i32, i32),
}

fn main() {
    // option T:
    let _some_number = Some(5);
    let _some_char = Some('e');
    // asigns some_number and char ...
    // ...the enum variable type from NullOption

    let _absent_number: Option<i32> = None;

    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
    println!("Hello world!");
    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("..1"));
}

// match expressions:
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Coin");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn main2() {
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
}

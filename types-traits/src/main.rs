#[allow(dead_code)]
fn main() {
    let _x = 0;
}
fn _remove_duplication() {
    let _number_list = vec![34, 50, 25, 100, 65];

    let mut _largest = &_number_list[0];
    for _number in &_number_list {
        if _number > _largest {
            _largest = _number;
        }

        println!("The largest number is {_largest}");
    }

    // above program will print the largest number from vector by duplicating
    // which is very tedious and inefficient
    // so:
}
fn _largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
fn _get_largest() {
    let _number_list = vec![34, 50, 25, 100, 65];
    let _largest1 = _largest(&_number_list);
    println!("The largest number is {_largest1}");
}
// type means the type of variable like i32, etc

/*------Generics------*/

// In Functions:

fn _largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn _largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn _inefficient_generics() {
    let _number_list = vec![34, 50, 25, 100, 65];
    let _result = _largest_i32(&_number_list);
    println!("The largest number is {_result}");

    let _char_list = vec!['y', 'm', 'a', 'q'];
    let char_result = _largest_char(&_char_list);
    println!("The largest char is {char_result}");
}
// above method is inefficient
// so we use type T as it accepts any type:

/*
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main123() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}
*/

//In Structs:

struct _Point<T> {
    x: T,
    y: T,
}

fn _main1234() {
    let _integer = _Point { x: 5, y: 10 };
    let _float = _Point { x: 1.0, y: 1.0 };
}
/*--------DIVIDER--------*/
struct _Point2<T, U> {
    x: T,
    y: U,
}

fn _main1() {
    let _mismatching = _Point2 { x: 5, y: 1.0 };
    // only works if there is T and U
}

// in methods:
struct _Point3<T> {
    x: T,
    y: T,
}

impl<T> _Point3<T> {
    fn _x(&self) -> &T {
        &self.x
    }
}

fn _main12345() {
    let p = _Point3 { x: 5, y: 10 };

    println!("p.x = {}", p._x());
}

/*------Traits------*/

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    } // trait defined and pub allowed other crates to use it
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn _use_trait() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course you propably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

/*------Lifetimes------*/
fn _longerstring() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = _longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}
/*
wont compile as return value is unknown, hence the lifetimes are required
    fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
*/
fn _longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
/*--------Traits in Impls--------*/
struct _ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> _ImportantExcerpt<'a> {
    fn _level(&self) -> i32 {
        3
    }
}

fn main() {}
fn _something() {
    let _b: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7);
    /*
        let third: &i32 = &v[2];
        println!("The third element is {third}");
        // accesses the third element (index of 2)
        let mut third: Option<i32> = v.get(2);
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element"),
        }
    */
    // cannot push now as there is a reference to an item here
    // more docs on vectors: https://doc.rust-lang.org/nomicon/vec/vec.html
    /*

    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    ^
    causes code to panic as we are accessing non-existent elements

    let does_not_exist = v.get(100);
    does not panic and returns none (because of .get)

    */
    let b = vec![100, 32, 57];
    for i in &b {
        println!("{i}");
    }

    let mut c = vec![100, 32, 57];
    for i2 in &mut c {
        *i2 += 50;
    }
    // adds 50 to all values of vector c
}
enum _SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn _multiple_type_vector() {
    let _row = vec![
        _SpreadsheetCell::Int(3),
        _SpreadsheetCell::Text(String::from("blue")),
        _SpreadsheetCell::Float(10.12),
    ];
}

/*-----------------Strings-----------------*/
fn _random() {
    let data = "initial contents";

    {
        let _s = data.to_string();
    }

    // the method also works on a literal directly:
    {
        let _s = "initial contents".to_string();
    }
    // equivalent to:
    {
        let _s = String::from("initial contents");
    }
    {
        // since strings are UTF-8 encoded, the following all can be stored in them:
        let _hello = String::from("السلام عليكم");
        let _hello = String::from("Dobrý den");
        let _hello = String::from("Hello");
        let _hello = String::from("שלום");
        let _hello = String::from("नमस्ते");
        let _hello = String::from("こんにちは");
        let _hello = String::from("안녕하세요");
        let _hello = String::from("你好");
        let _hello = String::from("Olá");
        let _hello = String::from("Здравствуйте");
        let _hello = String::from("Hola");
    }
    {
        let mut s = String::from("foo");
        s.push_str("bar");
        // will set s to (foo+bar =) foobar
    }
    {
        let mut s1 = String::from("foo");
        let s2 = String::from("bar");
        s1.push_str(&s2);
        println!("s2 is {s2}");
        //prints foobar
    }
    {
        let mut s = String::from("lo");
        s.push('l');
        //make s lo+l or lol
    }
    {
        let s1 = String::from("Hello,");
        let s2 = String::from(" World!");
        let _s3 = s1 + &s2;
        // add function called, s1 can no longer be used anymore from here
        // makes _s3 into: Hello, World!
    }
    {
        let s1 = String::from("Tic");
        let s2 = String::from("Tac");
        let s3 = String::from("Toe");
        let _s = format!("{s1}-{s2}-{s3}");
        // use format! to make code more readable
    }
    {
        // 2 ways to iterate over strings:

        {
            for c in "Зt".chars() {
                println!("{c}");
            }
            // will result in:
            // З
            // t
        }
        {
            for c in "3t".bytes() {
                println!("{c}");
                //will print the different bytes they each occupy (differs according to char)
            }
        }
    }
}

/*----------Divider----------*/

use std::collections::HashMap;
// a hashmap is similar to a vector
// but uses a value to access elements

fn _hashmaps() {
    {
        let mut scores = HashMap::new();
        //creates a hashmap

        scores.insert(String::from("Blue"), 10);
        // has key as blue and value as 10
        scores.insert(String::from("Yellow"), 50);
        // has key as yellow and value as 50

        for (key, value) in &scores {
            println!("{key}: {value}");
            // output:
            // Yellow: 50
            // Blue: 10
        }
    }
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.entry(String::from("Yellow")).or_insert(50);
        // checks if there is a value for key yellow in scores, if no, then:
        // adds key value pair (dependent on the line) of Yellow and 50
        // .or_insert returns mutable reference to value of corresponding Entry key
    }
    {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1
        }
        println!("{map:?}");
        // prints:
        // {"world": 2, "hello": 1, "wonderful": 1}
    }
}

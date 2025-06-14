fn main() {
    let mut s = String::from("Hello");
    // creates a string that can change values
    s.push_str(", world!");
    // adds , world! to the end of the string
    println!("{s}");
    let s1 = String::from("Hello");
    //let s2 = s1;
    // println!("{s1}");
    // above comment will not compile as s1 is no longer existent, AKA s1 is blank
    // to print s1 after s2 is created:
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");
    // NOTE: strings are out of scope after the end of the function
    // (s1 & s2 are no longer in scope here)
    let a = String::from("hello");
    println!("{a}");
    // let len = calc_length(&a);
    // &a is a pointer
    // mut ref requires it to be (&mut a)
    println!("length of a is ");

    //
    //
    //
    //

    // Slice type notes:
    let mut s = String::from("Hello World");
    let word = first_word(&s);
    s.clear();
    println!("{word}");

    let rand_string = String::from("Hello World");
    let hello = &rand_string[0..5];
    let world = &rand_string[6..11];

    println!("{hello} {world}");

    let s = String::from("Hello World");
    let word = first_word_better(&s);
    println!("{word}");
}
/*

fn calc_length(s: &string) -> usize {
    // s: &string is expecting a pointer to a string
    // for mut ref, must be (s: &mut string)
    s.len()
}

*/

//
// Slice type notes:

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    // converts string to array of bytes
    for (i, &item) in bytes.iter().enumerate() {
        // iter returns each element in a collection
        // enumerate wraps result of iter and returns element as tuple
        if item == b' ' {
            return i;
        } // searching for byte representing space
    }
    s.len()
}

fn first_word_better(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

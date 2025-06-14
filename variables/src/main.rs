fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is {x}");
    // constants are always immutable and must declare the data type
    // everything must be uppercase
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The length of spaces is {spaces}");
    // no error ^

    //let mut spaces1 = "   ";
    //let spaces1 = spaces.len(); -> leads to an error
    // integer types:
    // signed (negative or positive)
    // i8 (8bit), i16 (16bit).. i128 (128bit), arch (isize)
    // unsigned (positive only)
    // u8 (8bit), u16 (16bit).. u128 (128bit), arch (usize)

    /*let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';*/

    //like arrays but different = tuple (used when elements are of different types)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //get values of tup
    let (e, g, f) = tup;
    let y = tup.0;
    println!("The first value of tup is {y}");
    println!("The value of g is {g}");
    println!("The value of f is {f}");
    println!("The value of e is {e}");
    //starts from 0, going upwards

    //arrays: (used when number of elements will not change)
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("{}", months[0]);
    let a: [u32; 5] = [1, 2, 3, 4, 5];
    // creates an array with 5 elements, all signed 32 bit and then asigns the 5 values
    let b = [3; 5];
    // creates an array with 5 elements and all are 3

    // How to access array values:
    let first = a[0];
    let second = b[1];
    println!("The value of first is {first}");
    println!("The value of second is {second}");
    another_function();
}

//functions in rust:

fn another_function() {
    println!("Another function.");
}

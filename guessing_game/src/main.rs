use rand::Rng;
//random number generator implementation
use std::io;
// input and output
use std::cmp::Ordering;
// include less than, greater than, equal to
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // rand::thread_rng() says get a random number
    // gen_range(1..=100) specifies the number generation range

    // println!("The secret number is {}", secret_number);
    // prints the secret number
    loop {
        //loop creates an infinite loop
        println!("Please input your guess.");

        let mut guess = String::new();
        //mutable variables = variables that can change values
        // variables are immutable by default (no need to say inmuttable variable with inmut)
        // String::new() makes guess into a new string, but empty

        io::stdin() // to get input of user on line
            .read_line(&mut guess)
            .expect("Failed to read line");
        // &mut to ensure reference is mutable and not immutable (default)
        // set guess variable to the input of user
        //read_line appends to string, not overwrite
        //.expect is to ensure the input is valid

        // part of prev line of code [can be written as .read_line(&mut guess).expect("Failed to read line");]

        let guess: u32 = match guess.trim().parse() {
            // .trim() removes excess stuff at the end due to \r\n (windows) or \n (mac)
            // parse() converts string to unsigned 32 bit number
            Ok(num) => num,
            Err(_) => continue,
        };
        // match ensures guess = the number variation
        // basically, creates another guess with the same value as guess but in a unsigned 32 bit number rather than a string
        // .trim() removes any excess stuff at the end due to \r\n (windows) or \n (mac)

        println!("You guessed: {guess}");
        // if expression (x+2) then must be outside curly braces, otherise, variable can be inside curly braces

        match guess.cmp(&secret_number) {
            //match is a case statement
            //guess.cmp(&secret_number) is the comparison of guess and secret_number
            //cmp stands for compare and the value in bracket is the variable to compare
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

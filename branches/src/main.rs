fn main() {
    let x = 7;
    if x < 5 {
        println!("statement returned true");
    } else {
        println!("statement returned false");
    }
    if x != 0 {
        println!("x is not zero");
    }
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    }
    let condition = true;
    let number2 = if condition {
        5
    } else {
        6 /* same type as other possibility */
    };
    println!("number2 is {}", number2);
    // ctrl + c to interrupt program
    let mut counter = 0;
    let result = loop {
        counter += 1;
        // adds one to counter value
        if counter == 10 {
            break counter * 2;
            // breaks out of loop and returns value of counter * 2 (20 in this case)
        }
    };
    println!("The result is {}", result);
    let mut count = 0;
    'counting_up: loop {
        // outer loop with label 'counting_up
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
                // breaks unlabelled loop
            }
            if count == 2 {
                break 'counting_up;
                // breaks 'counting_up loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count {count}");
}

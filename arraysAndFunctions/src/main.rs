fn main() {
    //returning_function(5);
    another_function(5);
    print_labeled_measurements(5, 'h');
    //let x = (let y = 6);
    //above code does not compile
    // let y = 6 does not return a value, thus the program doesn't compile

    let y = {
        // this a block of code that sets y to x+1
        let x = 3;
        x + 1
    };
    println!("{y}");
    /*let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    // usize is used for arrays as it can store a value of almost any size

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");*/
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value} {unit_label}");
}
// fn reutrning_function(x: i32) -> i32 {
// x + 1
//x + 1;
// will not compile due to (;) at the end
// }

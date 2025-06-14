fn main() {
    println!("Error Handling");
}
use std::fs::File;

use std::io::ErrorKind;
// allows us to see the kind of error

fn _notes() {
    {
        // the result enum:
        /*
            enum Result<T, E> {
                Ok(T),
                Err(E),
            }
        */

        let _greeting_file_result = File::open("hello.txt");
        // return type of File::open is Result<File OR T, Error or E>
        // T will be whatever we get from File::open
        // E means error

        let _greeting_file = match _greeting_file_result {
            Ok(file) => file,
            // if file found, return file
            Err(error) => panic!("Problem opening the file: {error:?}"),
            // if file not found, panic with message
        };
    }
    {
        let _greeting_file_result = File::open("hello.txt");

        let _greeting_file = match _greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {e:?}"),
                },
                other_error => panic!("Problem opening the file: {other_error:?}"),
            },
            // You can match the type of error to what it should do/ how it should panic
        };
        /*
        Alternatively:
        .unwrap() will panic with the message if there is an error
        .expect("message") will give custom panic message
        */
    }
}

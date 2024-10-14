use std::fs::File;
// use std::fs::remove_file;
use std::io::ErrorKind;

fn main() {
    // Two types of errors in Rust : Unrecoverable and Recoverable
    // Recoverable : eg : File not found
    // Unrecoverable : eg : Trying to access a location beyond the end of a array and so we want immediatly stop the program

    //panic!("crash and burn");

    // // Exemple of unrecoverable error
    // let v = vec![1,2,3]; 
    // v[99];                      // To compile with BACK_TRACE : $ RUST_BACKTRACE = 1 cargo run

    // // Exemple of recoverable error

    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => println!("File {file:?} has been open"),
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => println!("File {fc:?} has been created"),
    //             Err(e) => panic!("Problem while creating the file : {e:?}"),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {other_error:?}");            
    //         }         
    //     },
    // };

    // remove_file("hello.txt");


    // // or 

    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {error:?}");
    //         })
    //     } else {
    //         panic!("Problem opening the file: {error:?}");
    //     }
    // });

    // remove_file("hello.txt");

}

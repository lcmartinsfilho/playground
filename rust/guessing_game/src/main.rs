// prelude
// use io library from standard library
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // creates a new mutable string 
    // String type
    // :: is an associated function of the String type (static method)
    // String::new returns a growable UTF-8 encoded string 
    // let mut guess = String::new();

    // read_line method on the std input handle (Stdin)
    // read_line returns a Result type, that is an enumeration (enum), 
    // which has a fixed set of values, called variants.
    // On this case, the possible results are Ok and Err
    // these error types serve to 
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line"); // Err

    println!("Your guessed: {}", guess);
}

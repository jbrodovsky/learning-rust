// Errors happen. In many cases Rust requires you to acknowledge the possibility of an error and handle it. 
// Rust has two categories of errors: recoverable and unrecoverable errors. Recoverable errors are just
// problems we would like to report to the user. Unrecoverable errors are always symptoms of bugs. Rust
// doesn't have exceptions. Instead it has the Result<T, E> for recoverable errors and panic! for 
// unrecoverable errors.

// Panic! is a macro that stops execution of the program and prints an error message. It can either unwind
// the stack or abort the program. The default behavior is to unwind, but you can change this by adding
// [profile] panic = 'abort' to your Cargo.toml file. This will make the program immediately abort and
// not unwind the stack. 

use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    println!("Hello, errors!");

    // A simple panic!
    // let v = vec![1, 2, 3];
    // v[99]; // This will panic!

    // In C/C++ attempting to read beyond the end of an array is undefined behavior. You might get a segfault
    // or you might get whatever is at the location in memory. This is a buffer overread and can lead to 
    // security vulnerabilities in your code. Rust instead will stop execution and refuse to continue.

    // Recoverable errors are handled with the Result<T, E> enum. This is a generic enum with two variants:
    // Ok(T) and Err(E). The T and E are generic types. The Ok variant holds a value of type T and the Err
    // variant holds a value of type E.

    let greeting_file_result = File::open("hello.txt"); // This will return a Result<File, io::Error>
    // To get the actual file we need to match on the Result
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        }
    };
    // We can also take different actions based on the error
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") { // If the file doesn't exist, create it
                Ok(file) => file,
                Err(error) => { // If we can't create the file, panic!
                    panic!("Problem creating the file: {:?}", error);
                }
            },
            other_error => {    // If the error is something else, panic!
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };
}

// When a function's implementation calls something that might fail, the function
// can handle the error itself, or return the error to the calling code. This is
// called propagating the error. The calling code can then decide how to handle
// the error. The below code returns a Result where the Ok variant holds the username
// and the Err variant holds the error. The calling code can then decide what to do
// with the error if one arises.
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("Hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}

// Alternatively we can use the ? operator to propagate the error.

fn read_username_from_file_alt() -> Result<String, io::Error> {
    // The ? operator is different from the match expression in that it will convert 
    // the error into the return type of the function using the From trait. 
    let mut username_file: File = File::open("hello.txt")?;
    let mut username: String = String::new();
    username_file.read_to_string(&mut username)?;
    return Ok(username);
}

// We can also use the ? operator on functions that return an Option<T> instead of a Result<T, E>.
// The ? operator will behave similarly. If the Option is None, the ? operator will return None early.
// If the Option is Some(T), the ? operator will return the T.
fn last_char_of_first_line(text: &str) -> Option<char> {
    return text.lines().next()?.chars().last();
    // Returns Option<char> because it is possible that ther is a character there, but it is also possible
    // that there is not. If there is not, the function will return None.
}
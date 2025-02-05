fn main() {
    println!("Hello, world!");
    another_function();
    function_with_parameters(5);
    function_with_multiple_parameters(5, 10, 'm');
    // Rust differentiates between statements and expressions. Expressions return a value, while statements do not.
    // - Statements are terminated with a semicolon, while expressions are not.
    let x = 5; // is a statement.
    println!("Statement: let x = {}", x);
    let y = { let x = 3; x + 1 }; // contains an expression (x + 1)
    println!("Expression: y = {}", y);
    // Functions can return values as either the last expression in the function or with the return keyword.
    let z = return_value(3);
    println!("Return value: z = {}", z);
    let a = use_return_keyword(5);
    println!("Return keyword: a = {}", a);
    // --- Control Flow ---
    let number = 3;
    if number < 5 { // conditions MUST be a boolean; Rust will not automatically convert 1/0 to true/false.
        println!("Condition is true.");
    } else {
        println!("Condition is false.");
    }
    // We can have multiple conditions with else if.
    let number = 6;
    if number % 4 == 0 {
        println!("Number is divisible by 4.");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3.");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2.");
    } else {
        println!("Number is not divisible by 4, 3, or 2.");
    }
    // But it will only evaluate the first true condition. Multiple checks can be done with simple if statements
    


}

// Functions are declared with the fn keyword and do not have to be declared before they are called like in C/C++.
fn another_function() {
    println!("Another function.");
}

// Functions can take arguments/parameters.
fn function_with_parameters(x: i32) {
    println!("The value of x is: {}", x);
}

fn function_with_multiple_parameters(x: i32, y: i32, label: char) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("X + Y = {} {label}", x + y);
}

fn return_value(a: i32) -> i32 {
    5 + a * 2
}

fn use_return_keyword(a: i32) -> i32 {
    return 5 + a;
}
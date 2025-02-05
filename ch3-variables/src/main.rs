const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // Constants are always immutable and available in any scope
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
    // Mutability makes a variable mutable (changeable)
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // we can redeclare a variable with let and shadow the previous value
    let x = x + 1;
    {
        // Shadowing only occurs in the scope that it is declared in
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
    // We can use let to shawdow a variable and change its type
    let spaces = "   ";
    println!("The value of spaces is: {spaces}");
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
    // However, mutability is not allowed
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // ^ Will not compile

    // Data types
    // Rust is a statically typed language, which means that it must know the types of all variables at compile time
    // Rust can usually infer the type of a variable, but we can also explicitly declare the type and in some cases it is required
    // let guess = "42".parse().expect("'Not a number!");
    // ^ Will not compile as it needs a type like below:
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {guess}");
    // Scalar types
    // - Integers
    // -- Rust has signed and unsigned integers that can be 8, 16, 32, 64, 128 bits in size. 
    // -- Signed integers can be positive or negative, while unsigned integers can only be positive and default to i32
    // -- The isize and usize types depend on the architecture of the computer the program is running on (32-bit or 64-bit)
    // - Floating-point numbers
    // -- Rust has two types of floating-point numbers: f32 and f64 and defaults to f64
    // - Mathematical operations
    // --- Addition: +
    // --- Subtraction: -
    // --- Multiplication: *
    // --- Division: /
    // --- Remainder: %
    // - Boolean
    // -- Rust has a bool type with two values: true and false
    // - Character
    // -- Rust has a char type that represents a single Unicode character and is specified with single quotes: 'z'
    // Compound types
    // - Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // -- Tuple elements can be accessed by using a period and the index of the element
    println!("The value of tup is: ({0}, {1}, {2})", tup.0, tup.1, tup.2);
    // -- Destructuring a tuple
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    // -- Tuple elements DO NO have to be the same type
    let new_tup = (500, "hello", 6.4);
    println!("The value of new_tup is: ({0}, {1}, {2})", new_tup.0, new_tup.1, new_tup.2);
    // - Array
    // -- Rust has an array type that requires a fixed size and all elements MUST be of the same type
    let a = [1, 2, 3, 4, 5];
    // -- Array elements can be accessed by using square brackets and the index of the element
    println!("The first value of a is: {0}", a[0]);
    // -- Array's primary use is to have a collection of items of the same type and a known fixed size
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("The third month is: {0}", months[2]); // <- Zero-based index
    // -- We can annotate the type and size of an array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a is: [{0}, {1}, {2}, {3}, {4}]", a[0], a[1], a[2], a[3], a[4]);
    // -- We can initialize an array with the same value for each element
    let a = [3; 5];
    println!("The value of a is: [{0}, {1}, {2}, {3}, {4}]", a[0], a[1], a[2], a[3], a[4]);
}

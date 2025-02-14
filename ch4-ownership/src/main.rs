fn main() {
    // Ownership
    // Computers have memory on both the stack and the heap.
    // Stack: fast, but limited in size; last in, first out
    // Heap: slower, but can grow in size; last in, first out
    // Memory on the heap is accessed through pointers, which 
    // are stored on the stack.
    // Ownership is Rust's system for managing memory on the 
    // heap. It has a set of rules that the compiler checks at
    // compile time. These rules are:
    // 1. Each value in Rust has a variable that is its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be
    //    dropped. Scope is the range within which a variable
    //    is valid. Easiest to see with curly braces.

    let s = "Hello, I'm a string literal"; // s is a string literal, which is immutable
    println!("{s}");
    let mut s = String::from("hello"); // s is a String, which is mutable when declared as such
    println!("{}", s);
    s.push_str(", world!");
    println!("{s}");
    // We can manage scope 
    {
        let s = String::from("hello scope");
        println!("{}", s);
    } // s goes out of scope here, and the memory is freed
    // by calling a built-in function called 'drop'
    println!("{}", s); // s in this scope is still 'hello, world!'
    // Multiple variables can interact with the same data
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);
    // This binds 5 to x, then copies the value of x to y.
    // This is because integers are stored on the stack, and
    // are simple values.
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); // This will not work, as s1 has been moved to s2 and the memory for s1 has been freed
    // We can however clone the data
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);
    // This is because the clone method creates a copy of the data
    // This seems to contradict the previous example with integers, 
    // but the difference is that the data on the heap is being copied, 
    // not the pointer to the data. In a way it's automatically cloning 
    // This is enabled by the 'Copy' trait, which is implemented for simple 
    // values like integers, but not for heap data. If a type has the Copy
    // trait, an assignment will copy the value. If it doesn't or instead 
    // has implemented the 'Drop' trait, the value will be moved.
    // - Types that implement the Copy trait:
    //   - All the integer types, like u32
    //   - The boolean type, bool, with values true and false
    //   - All the floating point types, like f64
    //   - The character type, char
    //   - Tuples, if they only contain types that also implement Copy

    // Ownership and functions
    // Passing a variable to a function will move or copy the value
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_ownership_and_gives_back(s2);
    println!("s1: {}, s3: {}", s1, s3);
    // Returning values from functions can also move or copy
    let s1 = String::from("hello");
    let (s2, length) = calculate_length(s1);
    println!("The length of '{s2}' is {length} characters.");
    // Passing and returning is a bit tedious, so we can use references
    // to pass a value by reference, which doesn't move the value
    let s1 = String::from("hello references");
    let length = calculate_length_by_reference(&s1);
    println!("The length of '{s1}' is {length} characters.");
    // Creating a references is refered to as borrowing, and the
    // reference is immutable by default. We can create mutable
    // references, but only one mutable reference is allowed at a time
    let mut s = String::from("hello");
    change(&mut s); // We need to pass a mutable reference to change
    println!("{}", s);
    // Of note, the mutable reference does not need to be returned to capture,
    // the mutated value. 

    // Slices
    // Slices are references to a part of a string, array, or other collection.
    let word = first_word(&s);
    println!("The first word is {word} characters long");
    let slice = &s[0..word]; // This is a slice of the first word
    println!("The first word is {slice}");
    // Slices are useful for passing parts of a collection to a function
    // without needing to copy the data.
    let slice = &s[..3]; // This is a slice of the first three characters
    println!("The first three characters are {slice}");
    let slice = &s[3..]; // This is a slice of the string from the fourth character to the end
    println!("The string from the fourth character is {slice}");
}
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    return some_string;
}
fn takes_ownership_and_gives_back(some_string: String) -> String {
    return some_string;
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    return (s, length);
}
fn calculate_length_by_reference(s: &String) -> usize {
    return s.len();
}
fn change(s: &mut String) { // We need to declare the reference as mutable too
    s.push_str(", world! I've been changed!");
}
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}
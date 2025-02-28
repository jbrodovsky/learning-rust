fn main() {
    // Storing lists of values with vectors
    let v: Vec<i32> = Vec::new(); // create an empty vector; note the type annotation since we didn't insert any values
    // More frequently, you'll need a vector with some data in it. We can use the vec! macro to create a vector that has initial values:
    let v = vec![1, 2, 3]; // create a vector with three elements
    // We can add elements to a vector by using the push method
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v); // [5, 6, 7, 8]
    // Reading elements of vectors
    let third: &i32 = &v[2]; // get a reference to the third element by indexing
    println!("The third element is {}", third);
    let third: Option<&i32> = v.get(2); // get a reference to the third element by using the get method
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    // Rust provides both of these ways to access an element in a vector to help you ensure that your program doesn’t try to access an element that doesn’t exist.
    // If you try to access an element at an index that the vector doesn’t have, your program will exit with a panic.
    // If you use get, your code will compile, but it will return None at runtime if you try to access an element out of bounds.
    // Iterating over the values in a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    // We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements:
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    
}

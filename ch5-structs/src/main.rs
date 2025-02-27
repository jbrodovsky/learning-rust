// Structs contain a collection of fields. Each field has a name and a type. Structs are 
// similar to tuples, but with named fields and no need to rely on the order of the fields.
// A struct's name should describe the meaning of the data it holds. Structs can be used to
// create custom data types.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // Create an instance of the User struct is pretty similar to declaring a variable.
    // The struct's name is followed by curly braces. Inside the curly braces, we specify
    // the field names and their values.
    let user1: User = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };
    // to get a specific value from a struct, we can use dot notation.
    println!("Username: {}", user1.username);
    // Structs, like all types in Rust, are immutable by default. To change a value in a
    // struct, we need to use the mut keyword.
    let mut user2: User = User {
        active: false,
        username: String::from("user2"),
        email: String::from("user2@example.com"),
        sign_in_count: 0,
    };
    println!("User2 email: {}", user2.email);
    user2.email = String::from("anewemail@mydomain.com");
    println!("User2 new email: {}", user2.email);
    // The entire stuct must be marked as mutable, not just the field we want to change.
    // We can also use a function to build a struct.
    let user3: User = build_user(String::from("user3@rust.org"), String::from("user3"));
    println!("User3 email: {}", user3.email);
    println!("User3 username: {}", user3.username);
    // This is similar to the constructor pattern in other languages.
    // We can also create a new instance of a struct from an existing instance and change
    // some values using the struct update syntax.
    let user4 = User {
        email: String::from("user4@rust-lang.org"),
        // the .. syntax specifies that the remaining fields should have the same values as
        // the fields in the instance we're copying from.
        ..user3
    };
    print_user(user4);
    // We can also create structs with no named fields to create different types of structs.
    // These are called tuple structs.
    // -------------------------
    //  struct Color(i32, i32, i32);
    //  let black = Color(0, 0, 0);
    // -------------------------
    // You can also define structs without any fields. These are called unit-like structs.
    // They are useful when you need to implement a trait on a type but don't have any data
    // that you want to store in the type itself.
    // -------------------------
    // struct EmptyStruct;
    // let empty = EmptyStruct;
    // -------------------------
    
}

fn build_user(email: String, username: String) -> User {
    // It makes sense to use the same name for the parameters and the struct fields.
    // We can use the field init shorthand to make the code more concise.
    // This:
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
    // is equivalent to this:
    // User {
    //     active: true,
    //     username: username,
    //     email: email,
    //     sign_in_count: 1,
    // }
}

fn print_user(user: User) {
    println!("Username: {}", user.username);
    println!("Email: {}", user.email);
    println!("Active: {}", user.active);
    println!("Sign in count: {}", user.sign_in_count);
}
// Enumerations
// Enumerations or enums allow you to define a type by enumerating its possible values.
// Structs give you a way of grouping related data together. Enums allow you to define
// a type by enumerating its possible values. For example, we migh have structs Circle,
// Rectangle, Triangle, etc. to represent different shapes. We could then define an enum
// called Shape that would hold values of these different types.
// -----------------
// We'll examine this via an IP address example. An IP address can be either a version 4
// or version 6 address. We can define an enum to represent this:
enum IpAddrKind {
    V4(String), // By including a String value, we can store the IP address as a string
    V6(String),
}
// This removes the need for a separate struct to hold the IP address.
// We can also use different types for the data attached to the enum variants:
enum IpAddr {
    V4(u8, u8, u8, u8), // This allows us to store the IP address as four u8 values
    V6(String),
}
// We can also define methods on enums:
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // This variant has an anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("Message::call()");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {           // Matches must be exhaustive, so we must handle all possible cases
        None => None,   // Particularly the None case when using Option<T>
        Some(i) => Some(i + 1),
    }
}

fn dice_roll(roll: i8) {
    match roll {
        2 => println!("Loser!"),
        3 => println!("Loser!"),
        7 => println!("Winner!"),
        11 => println!("Winner!"),
        12 => println!("Loser"),
        _ => println!("Point set: {}", roll) // all other values
    }
}

fn main() {
    // We can then create instances of the enum like this:
    let four: IpAddrKind = IpAddrKind::V4(String::from(""));
    let six: IpAddrKind  = IpAddrKind::V6(String::from(""));
    // We can also attach data to the enum variants. For example, we can store the IP address
    // as a string:
    let home: IpAddrKind = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback: IpAddrKind = IpAddrKind::V6(String::from("::1"));
    
    let m = Message::Write(String::from("hello"));
    m.call();

    // A common use of enums is to create Option<T>:
    // enum Option<T> {
    //    Some(T),
    //    None,
    // }
    // This is a generic enum, meaning it can hold values of any type. The Option<T> enum is
    // included in the standard library. It is used to represent the absence of a value. For
    // example, if we have a function that returns a number, but it might fail, we can use
    // Option<T> to handle the case where the function fails. The Some(T) variant holds a value
    // of type T, and None represents the absence of a value. This is a memory-safe way to handle
    // null values.
    let some_number: Option<i32> = Option::Some(5);
    // A simple way to get the underlying <T> value is to use the unwrap() method.
    println!("{}", some_number.unwrap());
    // But using a match expression is a better way to handle the case where the value is None.
    let x = Some(5);
    let y = plus_one(x);
    let z = plus_one(None);
    println!("x: {:?}", x);
    println!("y: {:?}", y);
    println!("z: {:?}", z);
    // Using enums and match expressions we can also take actions for a few specific values
    // and one default action for all others:
    dice_roll(7);
    dice_roll(2);
    dice_roll(5);
    dice_roll(11);
    dice_roll(12);
    dice_roll(4);
    // For simpler matching expressions, we can use if let:
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // An else condition can also be added to handle the false cases:
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("Not three");
    }
    // This can lead to overly verbose code, so an alternative is the let-else pattern:
    let val: Option<i32> = Some(8) else { return ();};
    println!("val: {:?}", val);

}

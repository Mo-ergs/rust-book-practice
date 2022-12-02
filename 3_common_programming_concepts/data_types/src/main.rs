// SCALAR TYPES
// A scalar type represents a single value.
// Rust has four primary scalar types:
// 1) integers
// 2) floating-point numbers
// 3) Booleans
// 4) characters

// BOOLEAN TYPE

// CHARARCTER TYPE

// COMPOUND TYPES

// tuples
// tuples have a fixed length and once declared cannot grow or shrink in size
// each position in the tuple has a type, each value can be of a different type
fn tuples() {
    // tuple set with explicit types
    let tuple_explicit: (i32, f64, u8) = (500, 6.4, 1);

    // get values out of a tuple by destructuring
    let (x, y, z) = tuple_explicit;

    println!("the value of x = {x}");
    println!("the value of y = {y}");
    println!("the value of z = {z}");

    // get values out of a tuple by using . notation
    println!("the value of x = {}", tuple_explicit.0);
    println!("the value of y = {}", tuple_explicit.1);
    println!("the value of z = {}", tuple_explicit.2);
}

// ARRAYS
// every element of an array must be of the same type
// array length is also fixed after declaration just like tuples
fn arrays() {
    let a = [1, 2, 3, 4, 5];
    println!("a {:?}", a);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("months {:?}", months);

    let b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a {:?}", b);

    // accessing array elements
    let first = a[0];
    let second = a[1];
    println!(" first:{first} second:{second}");
}

fn main() {
    tuples();
    arrays();
}

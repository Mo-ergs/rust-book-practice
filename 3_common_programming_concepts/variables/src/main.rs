// VARIABLES AND MUTABILITY

// variables are immutable by default, if the following code is uncommented it will result in an error

// fn immutable_variable() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// use mut before a variable to make it mutable
fn mutable_variable() {
    let mut x = 5;
    println!("the value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");
}

// CONSTANTS
fn constant() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds = {THREE_HOURS_IN_SECONDS}")
}

// SHADOWING

// by shadowing a variable, mutations to the variable are made before returning the variable to an immutable state
fn shadowing_change_value() {
    let x = 5;
    println!("Unshadowed x = {x}");

    let x = x + 1;
    println!("Shadowed x = {x}");

    {
        let x = x * 2;
        println!("The value of inner scope shadowed x = {x}");
    }

    println!("Shadowed x = {x}");
}

// shadowing also permits a change the type of a value stored in a variable
fn shadowing_change_value_type() {
    let spaces = "   ";
    println!("spaces is a string here {spaces}");

    let spaces = spaces.len();
    println!("spaces has changed to be a number here {spaces}");
}

fn main() {
    mutable_variable();
    constant();
    shadowing_change_value();
    shadowing_change_value_type();
}

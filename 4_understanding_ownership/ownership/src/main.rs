// Each value in Rust has a variable that’s called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.
#[allow(dead_code)]
fn variable_scope() {
    {
        let _s = "hello"; // scope of s starts here, s is valid from this point on

        // lines of code doing stuff with s
    } // scope of s ends here, s is not longer valid
}

fn data_move_integer() {
    let x = 5;
    let y = x;

    // this is valid because integers are on the stack
    println!("The value of x is {x} and the value of y is {y}");
}

fn data_move_string() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    // this would only be valid with clone, because strings are stacked on the heap
    println!("The value of s1 is {s1} and the value of s2 is {s2}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn main() {
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    data_move_integer();
    data_move_string();
}

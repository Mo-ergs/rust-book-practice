// the type of a parameter in a function's signature MUST be declared
fn print_number(x: i32) {
    println!("the value of x is {x}");
}

// multiple parameters need to be separated by,
fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("the measurement is {value}{unit_label}");
}

// function bodies are made up of statements and expressions
// statements perform some action, do not return a value, must end with ;
// expressions evalutate to value, do not end with ;
fn explain_statements_and_expressions() {
    // statement
    let x = 6;

    let y = {
        let x = 3;
        x + 1
    };

    println!("the value of x is {x} and y is {y}");
}

// function with return values
// functions can return values to the code that calls them
fn five() -> i32 {
    5
}

fn main() {
    print_number(10);
    print_labeled_measurements(5, 'g');
    explain_statements_and_expressions();
    let x = five();
    println!("x is {x}");
}

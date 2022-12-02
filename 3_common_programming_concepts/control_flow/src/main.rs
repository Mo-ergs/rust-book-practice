fn loop_in_a_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}

// returning a value from a loop
fn loop_return_value() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result of the loop is {result}");
}

// while loop
// PREFER USING FOR LOOPS!!!!!!!!!!!!!!
fn while_loop() {
    let mut num = 3;

    while num != 0 {
        println!("{num}");
        num -= 1;
    }

    println!("LIFTOFF!!!");
}

// looping through a collection
fn iterate_collection() {
    let a = [10, 20, 30, 40, 50];

    for i in a {
        println!("{i}");
    }
}

fn iterate_collection2() {
    for i in (1..4).rev() {
        println!("{i}")
    }
    println!("LIFTOFF!!!!")
}

fn main() {
    loop_in_a_loop();
    loop_return_value();
    while_loop();
    iterate_collection();
    iterate_collection2();
}

fn fibonacci_sequence() {
    let mut fibonacci_sequence: Vec<i32> = Vec::new();
    let mut counter = 0;

    while counter < 21 as usize {
        if counter == 0 {
            fibonacci_sequence.push(counter as i32);
            counter += 1;
            continue;
        } else if counter == 1 {
            fibonacci_sequence.push(counter as i32);
            counter += 1;
            continue;
        } else {
            fibonacci_sequence
                .push(fibonacci_sequence[counter - 1] + fibonacci_sequence[counter - 2]);
            counter += 1;
        }
    }

    println! {"fibonacci sequence to 20 is: {:#?}", fibonacci_sequence}
}

fn main() {
    fibonacci_sequence()
}

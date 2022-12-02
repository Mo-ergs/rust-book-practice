// creating a refernce is called borrowing and is immutable by default
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn demonstrate_calculate_length() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("the length of {} is {}", s1, len);
}

// creating a mutable borrow
fn change_borrow(s: &mut String) {
    s.push_str(", world");
}

fn demonstrate_change_borrow() {
    let mut string = String::from("hello");
    change_borrow(&mut string);
    println!("{} has been changed", string);
}

// there can only be one mutable reference to piece of data at anytime
// can seperate the scope of mutable borrows in order to achieve it
#[allow(dead_code)]
fn multiple_mutable_references() {
    let mut s = String::from("hello");

    {
        let _r1 = &mut s;
    }
    let _r2 = &mut s;
}

fn main() {
    demonstrate_calculate_length();
    demonstrate_change_borrow();
}

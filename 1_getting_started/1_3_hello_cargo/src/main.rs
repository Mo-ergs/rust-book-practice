fn is_palindrome(x: String) -> bool {
    let rev_x = x.chars().rev().collect::<String>();
    if x == rev_x {
        return true;
    } else {
        return false;
    }
}

fn main() {
    is_palindrome(String::from("jake"));

    // let number = 3;

    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }
}

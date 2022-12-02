fn main() {
    // both of these do the same thing
    let s = "initial contents".to_string();
    let _r = String::from("initial contents");

    // Updating a String
    // with push_str
    let mut a = String::from("intial contents");
    a.push_str("with more stuff");

    // with format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let combined_string = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", combined_string);

    // Methods for iterating over strings and getting parts of strings
    for i in s.chars() {
        println!("{}", i);
    }
}

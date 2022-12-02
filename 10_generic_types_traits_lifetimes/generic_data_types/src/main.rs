// write a function that finds the largest number in a list
fn find_largest_number(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

// WAYS OF USING GENERIC TYPE PARAMETERS

// structs
// struct with 1 generic type parameter
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// struct with 2 generic type parameters
#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

// method on struct with generic type parameters
impl<T> Point<T> {
    fn print_x(&self) -> &T {
        &self.x
    }
}

//enums
//  enum with one generic paramter
enum Option<T> {
    Some(T),
    None(),
}

// enum with two generic parameters
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let number_list = vec![25, 13, 19, 27, 32, 88, 101];
    println! {"largest number in number list is: {}", find_largest_number(&number_list)};

    let number_list = vec![50, 16, 21, 2, 900, 66, 81];
    println! {"largest number in number list is: {}", find_largest_number(&number_list)};

    let both_integer = Point { x: 2, y: 5 };
    let both_float = Point { x: 4.5, y: 6.5 };

    println!("{:#?}", &both_integer);
    println!("{:#?}", both_float);
    println!("x is {}", Point::print_x(&both_integer));
    println!("x is {}", &both_integer.print_x());

    let integer_and_float = Point2 { x: 34, y: 5.5 };
    println!("{:#?}", integer_and_float);
}

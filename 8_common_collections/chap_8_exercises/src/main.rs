use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn find_mean(numbers: &Vec<i32>) -> i32 {
    let total: i32 = numbers.iter().sum();

    let mean = total / numbers.len() as i32;

    mean
}

fn find_median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort();

    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        let median = numbers[mid - 1] + numbers[mid] / 2 as i32;
        median
    } else {
        numbers[mid]
    }
}

fn find_mode(numbers: &mut Vec<i32>) -> String {
    let mut map = HashMap::new();

    for number in numbers {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }

    let max_value = map.values().cloned().max().unwrap_or(0);

    let mut max_value_string = String::from("");

    for (key, value) in map {
        if value == max_value {
            max_value_string = format!("mode is {} which occurs {} times", key, value);
        }
    }

    max_value_string
}

fn create_pig_latin(word: &mut String) -> String {
    let first_char = word.remove(0);

    match first_char {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-{}", word, "hay"),
        _ => format!("{}-{}", word, "ay"),
    }
}

enum Command {
    Add {dept: String ,name: String },
    List(String),
    All,
    Quit,
}

impl Command  {
    fn parse_input_string(string: &str) -> Option<Self> {
        let words: Vec<&str> = string.trim().split_whitespace().collect();
        println!("{:?}",words.as_slice());
        match words.as_slice() {
            ["Add", name, "to", dept] => Some(Command::Add {dept: dept.to_string(), name: name.to_string()}),
            ["List", dept] => Some(Command::List(dept.to_string())),
            ["All"] => Some(Command::All),
            ["Quit"] => Some(Command::Quit),
            _ => None,

        }
    }
}

fn main() {
    let mut vector_list = vec![12, 2, 31, 69, 53, 69, 70, 8, 9, 101, 7];

    let mut consanant_string = String::from("godzilla");
    let mut vowel_string = String::from("america");

    println!("mean: {}", find_mean(&vector_list));

    println!("median: {}", find_median(&mut vector_list));

    println!("{:?}", find_mode(&mut vector_list));

    println!("as pig latin = {}", create_pig_latin(&mut consanant_string));

    println!("as pig latin = {}", create_pig_latin(&mut vowel_string));

    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    let stdin = io::stdin();
    println!("Type 'Add<name> to <department>' to add and employee to a department");
    println!("Type 'List <department>' to list the names of employees working in that department");
    println!("Type ''All' to list all employees by department");
    println!("Type 'Quit' to quit");
    for line in stdin.lock().lines() {
        let input = line.expect("error: unable to read user input");
        match Command::parse_input_string(&input) {
            Some(Command::Add { dept, name }) => employees.entry(dept).or_default().push(name),
            Some(Command::List(dept)) => match employees.get(&dept) {
                Some(names) => {
                    for name in names {
                        println!("{}: {}",dept, name);
                    }
                }
                None => println!("That department does not exist"),
            },
            Some(Command::All) => {
                for (dept, names) in &employees {
                    let mut names = names.clone();
                    names.sort();
                    for name in names {
                        println!("{}: {}", dept, name);
                    }
                }
            }
            Some(Command::Quit) => break,
            None => println!("input error")
        }
    }
    println!("goodbye");

}

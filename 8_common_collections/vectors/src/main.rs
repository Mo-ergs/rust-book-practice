fn main() {
    let mut v = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("Vector: {:?}", v);

    // GETTING VALUES FROM A VECTOR
    // index method
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // get method
    match v.get(2) {
        Some(value) => println!("the third element is {}", value),
        None => println!("there is no third element."),
    }

    // ITERATING OVER THE VALUES IN A VECTOR
    let c = vec![100, 32, 57];
    for i in &c {
        println!("{}", i);
    }

    // here * is the dereference operator
    let mut b = vec![100, 32, 57];
    for i in &mut b {
        *i += 50;
    }

    // Using an Enum to Store Multiple Types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}

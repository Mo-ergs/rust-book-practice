// Two ways to create hashmaps:
// 1 - create new hash map manually and insert keys and values
// 2 - create a hash map by using iterators and collect method on a vector of tuples

// there is no macro to create hashmaps and they have to bring in the collections library to be used.
use std::collections::HashMap;

fn main() {
    // 1 - create new hash map manually and insert keys and values
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 2 - create a hash map by using iterators and collect method on a vector of tuples
    let teams2 = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores2 = vec![10, 50];

    let mut _scores2: HashMap<_, _> = teams2
        .into_iter()
        .zip(initial_scores2.into_iter())
        .collect();

    // HASH MAPS AND OWNERSHIP
    // For types that implement Copy trait e.g. i32, the values are copied into the hash map
    // for owned values likes String, the values are moved into AND hash map becomes the owner

    // ACCESSING VALUES IN A HASH MAP
    // get a value out of a hasmap by providing its key to the get method

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);

    // ITERATING OVER A HASH MAP
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // UPDATING A HASHMAP

    // Overwriting a value
    // If we insert a key and value into a hashmap and then insert that same key with a different value,
    // the value associated with that key will be replaced.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Only Inserting a Value If the Key Has No Value
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

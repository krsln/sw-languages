use std::collections::HashMap;

pub fn run() {
    println!();
    println!("==================================================");
    println!("Collection Types [Vectors UTF8 Hashmaps]");
    println!("==================================================");

    println!("----- Vector");

    // Macro to create a vector of numbers
    let mut _v: Vec<i32> = Vec::new();
    let mut _v = vec![1, 2, 3];

    _v.push(5);
    _v.push(25);
    println!("_v = {:?}", _v);

    let vector = vec![1, 2, 3, 4, 5, 6, 7];
    let third: &i32 = &vector[2]; // Direct indexing
    println!("The third element is {}", third);
    let third = vector.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    println!("----- UTF8");
    let s = "whatever".to_string();
    let s = String::from("whatever");
    // Mutate the variable [push to it]
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    println!("s = {:?}", s);

    let salam = String::from("Здравствуйте");
    let salut: String = String::from("Salut");

    // If you want to combine strings, use the + operator
    let s1: String = String::from("Hello, ");
    let s2: String = String::from("world!");
    let s3: String = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("The value of s3 = {}", s3);

    // Formating String
    let full_message = format!("{} {}", salam, salut);
    println!("full_message = {}", full_message);

    println!("----- Hashmap");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("The score is {}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

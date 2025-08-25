// Error Handling Techniques [2 Approaches]
// Approach-1 Result<T,E>
// Approach-2 Option <T>

// Approach 1
// Define the generic option type
enum Option<T> {
    Some(T), // Represent a value
    None,    // Represent no value
}

fn divide_option(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        Option::None
    } else {
        Option::Some(numerator / denominator)
    }
}

// Approach 2
// Define the generic result type
enum Result<T, E> {
    Ok(T),  // Represent a value
    Err(E), // Represent an error
}
fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Result::Err(String::from("Divide by zero"))
    } else {
        Result::Ok(numerator / denominator)
    }
}


pub(crate) fn main() {
    println!();
    println!("==================================================");
    println!("Error Handling ");
    println!("==================================================");

    // Approach 1
    let result = divide_option(10.0, 0.0);
    match result {
        Option::Some(x) => println!("Result = {}", x),
        Option::None => println!("Cannot divide by Zero!"),
    }
    let result = divide_option(66.0, 26.0);
    match result {
        Option::Some(x) => println!("Result = {}", x),
        Option::None => println!("Cannot divide by Zero!"),
    }

    // Approach 2
    match divide_result(10.0, 0.0) {
        Result::Ok(result) => println!("Result = {}", result),
        Result::Err(err) => println!("Error = {}", err),
    }
    match divide_result(66.0, 26.0) {
        Result::Ok(result) => println!("Result = {}", result),
        Result::Err(err) => println!("Error = {}", err),
    }
}

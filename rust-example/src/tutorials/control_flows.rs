// Control Flows in Rust
// 1- Conditions [If-Else]
// 2- Repeating Actions [Loops]

pub(crate) fn main() {
    println!();
    println!("==================================================");
    println!("Control Flows");
    println!("==================================================");

    conditions();
    loops();
}

fn conditions() {
    println!();
    println!("----- Conditions [If-Else]");

    // If-Else [if expression] [else expression]
    let age: u16 = 18;
    if age >= 18 {
        println!("You can drive a car!");
    } else {
        println!("You can't drive a car! Age: {}", age);
    }

    // Multiple conditions wih else if:
    let number = 6;
    if number % 4 == 0 {
        println!("This number is divisible by 4");
    } else if number % 3 == 0 {
        println!("This number is divisible by 3");
    } else if number % 2 == 0 {
        println!("This number is divisible by 2");
    } else {
        println!("This number is not divisible by 4, 3, or 2");
    }

    // Using if in a let statement
    let condition = true;
    let number = if condition { 15 } else { 60 };
    println!("The value of number is: {}", number);
}

fn loops() {
    println!();
    println!("----- Repeating Actions [Loops]");

    println!("----- Loops [Loop]");
    // loop {
    //     println!("Hello, Loop!");
    //     // infinite without a break
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // Loob labels to disambiguate between multiple loops
    let mut count = 0;
    'counting_up: loop {
        println!("Counting down: {}", count);
        let mut remaining = 10;
        loop {
            println!("Remaining: {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
            count += 1;
        }
    }
    println!("----- Loops [While]");
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    println!("----- Loops [ForLoop]");
    // Loop through a collection with for loop
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
    let b = ["a", "b", "c"];
    for letter in b {
        println!("the letter is: {}", letter);
    }
}

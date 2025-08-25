//Functions
// Entry point
// a function / variables should be written in snake case
// kebab case: hello-world
// snake case: hello_world

pub fn functions() {
    hello_world();
    tell_height(186);
    human_id("Joel", 52, 182.3);

    expression();

    // calculate the BMI function
    let weight = 65.0;
    let height = 1.87;
    let bmi = calculate_bmi(weight, height);
    println!("your bmi = {:.2}", bmi);
}

// Hoisting - can call function anywhere in you code
fn hello_world() {
    println!("Hello, Rust🦀!");
}

// you can insert input values
fn tell_height(height: u32) {
    println!("My height is {} cm.", height);
}

// you can insert more than one value
fn human_id(name: &str, age: u32, height: f32) {
    println!(
        "My name is {}, I am {} years old and my height {} cm.",
        name, age, height
    );
}

// Expressions & Statements
fn expression() {
    // Expressions: Anything that returns a value.
    //-----
    // 5
    // true & false
    // add (3, 4)
    // if condition {value1} else {value2}
    // ({code})

    // expressions can be in code block
    let x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };
    println!("Result is {}", x);

    // ada (4, 6) ;
    let y: i32 = add(4, 6);
    println!("Value of y is : {}", y);
    println!("Value from function 'add' is: {}.", add(4, 6));

    // Statements: Anything that doesn't return a value.
    //-----
    // Almost all statements in Rust end with ;
    // let y = let x = 10;
    // 1 Variable declarations: let x = 5;
    // 2 Function definitions: fn foo@ {}
    // 3 Control flow statements: if condition { /* code */ }
    // else { /* code */}, while condition { /* code */ }, etc.
}

// function return a value
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// BMI = weight(kg)/height(m)^2
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}

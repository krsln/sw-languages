// Shadowing is not the same as marking a variable as mutable.

pub fn run() {
    println!();
    println!("==================================================");
    println!("Shadowing");
    println!("==================================================");

    let i = 5; // result is 5
    let i = i + 1; // result is 6

    {
        let i = i * 2; // result is 12
        println!("The value of x in the inner scope is: {}", i);
    }

    println!("The value of x is: {}", i);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces);
}

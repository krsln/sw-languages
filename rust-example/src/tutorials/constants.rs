// Constants

// You can declare a constant here with a type annotation
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

pub fn main() {
    println!();
    println!("==================================================");
    println!("Constants");
    println!("==================================================");
    
    let mut x = 5;
    // Can not be mutable
    const Y: i32 = 10;
    println!("The value of x is: {}", x);
    println!("The value of Y is: {}", Y);
    println!(
        "The value of THREE_HOURS_IN_SECONDS is: {}",
        THREE_HOURS_IN_SECONDS
    );
}

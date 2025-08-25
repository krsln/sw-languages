// Ownership Borrowing & Referencing
fn set_title(title: &str) {
    println!();
    println!("==================================================");
    println!("{title}");
    println!("==================================================");
}

pub fn run() {
    ownership();
    referencing();
    mutable_immutable_references();
    var_mutability();
}

// Ownership
//----------------
// C, C++ -> Memory Management Control Issue
// Garbage Collector solved this issue, but created a new issue
// -> Slow Performance: [stopping/Resuming the program]
// OWNERSHIP introduced by Rust to solve memory safety issues and high performance at the same time.
// What is Ownership ?
// Every value has a single owner [every variable has one value, and it is its sole owner].

pub fn ownership() {
    set_title("Ownership");

    // Ownership Rules
    //----------------
    // 1- Each value in Rust has a variable that's its owner.
    let s1 = String::from("RUST");
    let len = calculate_size(&s1);
    println!("The size of '{}' is {}.", s1, len);

    // 2- There can be only one owner at a time.
    let s2 = s1;
    // println!("{}", s1); // Value used after being moved [E0382]
    println!("{}", s2);

    // 3- When the owner goes out of scope, the value will be dropped.
    // -> print_lost
}

// // s1 goes out of scope and its value will be dropped
// fn print_lost(s: &String) {
//     println!("{}", &s1); //  cannot find value `s1` in this scope
// }

fn calculate_size(s: &String) -> usize {
    s.len()
}

// ==================================================

// Borrowing & Referencing
// Safety and Performance
// Borrowing and references are powerful concepts

// Understanding References
// References: Enable you to borrow values without taking ownership.
// Immutable Reference & Mutable Reference
// Create Reference by add "&"

pub fn referencing() {
    set_title("Borrowing & Referencing");

    // -I- Immutable Reference
    let _x = 5;
    let _r = &_x;
    println!("Value of _x: {}", _x);
    println!("Value of _r: {}", _r);

    // -II- Mutable Reference
    let mut _a = 5;
    let mut _b = &mut _a;
    *_b += 1;
    *_b -= 3;

    println!("Value of _a: {}", _a);
    // println!("Value of _b: {}", _b); // only one can be used
}

// ==================================================

// Demonstration on one mutable reference or many immutable references
pub fn mutable_immutable_references() {
    let mut account = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.36,
    };

    // Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to withdraw money
    account.withdraw_balance(40.00);

    // Immutable borrow to check the balance
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw_balance(&mut self, amount: f64) {
        println!(
            "Withdrawing from account {} owned by {}",
            amount, self.owner
        );
        self.balance -= amount;
    }
    fn check_balance(&mut self) {
        println!(
            "Account owned by {} has a balance of {}",
            self.owner, self.balance
        );
    }
}

// ==================================================
// Variables and Mutability
pub fn var_mutability() {
    let mut a = 5;
    println!("The value of a is: {}", a);
    a = 10;
    println!("The value of a is: {}", a);
}

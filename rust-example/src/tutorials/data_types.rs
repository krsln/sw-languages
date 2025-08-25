fn set_title(title: &str) {
    println!();
    println!("==================================================");
    println!("{title}");
    println!("==================================================");
}

pub fn run() {
    primitive_data_types();
    compound_data_types();
}

// ==================================================
// Primitive Data Types
// int float bool char

pub fn primitive_data_types() {
    set_title("Primitive Data Type");

    println!();
    println!("----- Integer");
    // Integer
    // # Rust has signed (+ and -) and unsigned integer (only+) types of different sizes.
    // i8, i16, 132, іб4, i128: Signed integers.
    // u8, u16, u32, uб4, u128: Unsigned integers.

    let x: i32 = -42;
    let y: u64 = 99;

    println!("Signed x: i32 -> {}", x);
    println!("Unsigned y: u64 -> {}", y);

    // diff bet i32 (32 bits) and i64(64 bits)
    // range :
    // i32 - 2147483647
    // 164 - 9223372036854775807
    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;
    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);

    println!();
    println!("----- Float");
    // Floats [Floating Point types]
    // f32 f64
    let pi: f32 = 3.14;
    println!("Value of Pi is '{pi}'");

    println!();
    println!("----- Bool");
    // Boolean Values: true false
    let is_snowing: bool = true;
    println!("Is it snowing? {is_snowing}");

    println!();
    println!("----- Char");
    // Character Type - char
    let letter: char = 'a';
    println!("First letter of the alphabet is {}", letter);
}

// ==================================================
// Compound Data Types
// arrays tuples slices strings (slice string)

pub fn compound_data_types() {
    set_title("Compound Data Type");

    println!("----- Arrays");
    // Arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array {:?}", numbers);
    let fruits: [&str; 3] = ["apple", "orange", "pear"];
    println!("Fruits Array {:?}", fruits);
    println!("Fruits Array 1st element -> {}", fruits[0]);
    println!("Fruits Array 2st element -> {}", fruits[1]);
    println!("Fruits Array 3st element -> {}", fruits[2]);

    println!();
    println!("----- Tuples");
    // Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple {:?}", human);
    let kratos = ("Kratos", 32, true, [1, 2, 3, 4, 5]);
    println!("My Mix Tuple {:?}", kratos);

    println!();
    println!("----- Slices");
    // Slices [1, 2, 3, 4, 5]
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number slices {:?}", number_slices);
    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal slices {:?}", animal_slices);
    let book_slices: &[&String] = &[
        &"IT".to_string(),
        &"Harry Potter".to_string(),
        &"ZEN".to_string(),
    ];
    println!("Book slices {:?}", book_slices);

    // String vs String Slices (&str)
    // A- strings [ growable, mutable, owned string type ]
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);
    // B- &str (String Slice)
    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);
}

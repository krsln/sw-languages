#![allow(unused)]

mod tutorials;

fn main() {
    let unused_number: i32 = 13;

    println!("Hello, 🦀 from CARGO!");

    tutorials();
}

fn tutorials() {
    tutorials::data_types::primitive_data_types();
    tutorials::data_types::compound_data_types();

    tutorials::functions::functions();

    tutorials::ownership::ownership();
    tutorials::ownership::referencing();
    tutorials::ownership::mutable_immutable_references();
    tutorials::ownership::var_mutability();

    tutorials::constants::main();
    tutorials::shadowing::main();
    tutorials::control_flows::main();
    tutorials::structs::main();
    tutorials::enums::main();
    tutorials::error_handling::main();
    tutorials::collection_types::main();
}

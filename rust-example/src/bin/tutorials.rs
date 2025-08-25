use rust_example::tutorials;

fn main() {
    println!("🦀This is the tutorials main.rs!");

    tutorials::data_types::run();
    tutorials::functions::run();
    tutorials::ownership::run();
    tutorials::constants::run();
    tutorials::shadowing::run();
    tutorials::control_flows::run();
    tutorials::structs::run();

    tutorials::enums::run();
    tutorials::error_handling::run();
    tutorials::collection_types::run();
}

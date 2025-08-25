// Structs are used to name and package related values similar to tuples.

pub fn run() {
    println!();
    println!("==================================================");
    println!("Structs ");
    println!("==================================================");

    // tuple
    let rect = (200, 200);
    println!("Tuple -> Rectangle {rect:?}");

    // struct
    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    let mut user1 = User {
        active: true,
        username: "someusername".to_string(),
        email: "someusername@m.com".to_string(),
        sign_in_count: 1,
    };

    user1.email = String::from("anothermail@m.com");
    println!("user1 email: {}", user1.email);

    // Return a struct from a function
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    // Create instance from other instances
    let user2 = User {
        email: String::from("another@m.com"),
        ..user1
    };
    println!("user2 email: {}", user2.email);

    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
    let origin = Point(0, 0, 0);

    // unit-like struct
    struct AlwaysEqual;
    let always_equal = AlwaysEqual;

}

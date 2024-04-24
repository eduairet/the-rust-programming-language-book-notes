fn main() {
    let mut user1 = User {
        username: String::from("dope"),
        email: String::from("sports@vb.punk"),
        active: true,
        sign_in_count: 1,
    };

    user1.username = String::from("super-dope");

    println!(
        "User1: {}, {}, {}, {}",
        user1.username, user1.email, user1.active, user1.sign_in_count
    );

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect) // Borrowing the reference to the struct will prevent the struct from being moved
    );
}

struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

// With tuples
//fn area(rectangle: (u32, u32)) -> u32 {
//    rectangle.0 * rectangle.1
//}

// With structs
#[derive(Debug)] // This is a debug print trait that can be used to print the struct
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    println!("Rectangle is {:?}", rectangle); // Debug print
    println!("Rectangle is {:#?}", rectangle); // Debug print with pretty print
    dbg!(rectangle); // Debug output
    rectangle.width * rectangle.height
}

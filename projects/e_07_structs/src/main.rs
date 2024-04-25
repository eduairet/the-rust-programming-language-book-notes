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

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area() // Using the method
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("Square is {:?}", sq);
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

impl Rectangle {
    // Area as a method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method to check if a rectangle can hold another rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

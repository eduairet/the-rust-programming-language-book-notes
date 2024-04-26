fn main() {
    let up = Direction::Up;
    let right = Direction::Right;
    let down = Direction::Down;
    let left = Direction::Left;
    println!(
        "Up: {:?} | Right: {:?} | Down: {:?} | Left: {:?}",
        up, right, down, left
    );

    let red = Color::Red(255);
    let green = Color::Green(255);
    let blue = Color::Blue(255);
    println!("Red: {:?} | Green: {:?} | Blue: {:?}", red, green, blue);

    let success = Status::Success;
    let error = Status::Error(404);
    println!("Success: {:?} | Error: {:?}", success, error);
    println!(
        "Success code: {} | Error code: {}",
        success.code(),
        error.code()
    );

    let some = Option::Some(42i32);
    let none = Option::<i32>::None;
    println!("Some: {:?} | None: {:?}", some, none);

    let quarter = Coin::Quarter(UsState::NewYork);
    println!("Quarter value: {}", value_in_cents(quarter));
    println!("Penny value: {}", value_in_cents(Coin::Penny));
    println!("Nickel value: {}", value_in_cents(Coin::Nickel));
    println!("Dime value: {}", value_in_cents(Coin::Dime));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Five: {:?} | Six: {:?} | None: {:?}", five, six, none);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("One"),
        3 => println!("Three"),
        5 => println!("Five"),
        7 => println!("Seven"),
        _ => (),
    }

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("Three");
    } else {
        println!("Not three");
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Color {
    Red(u8),
    Green(u8),
    Blue(u8),
}
// Equivalent to:
// struct Color {
//   color: String,
//   value: u8,
// }
// red = Color { color: "Red".to_string(), value: 255 }
// green = Color { color: "Green".to_string(), value: 255 }
// blue = Color { color: "Blue".to_string(), value: 255 }

#[derive(Debug)]
enum Status {
    Success,
    Error(u32),
}

impl Status {
    fn code(&self) -> u32 {
        match self {
            Status::Success => 200,
            Status::Error(code) => *code,
        }
    }
}

#[derive(Debug)]
enum UsState {
    NewYork,
    NewJersey,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}

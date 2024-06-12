fn main() {
    // match
    let x = 2;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("something else"),
    };

    let y = Some(5);
    match y {
        None => None,
        Some(i) => Some(i + 1),
    };

    // if let
    let x = Some(5);
    if let Some(5) = x {
        println!("five");
    } else {
        println!("not five");
    }

    // while let
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for loop
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let
    let (x, y, z) = (1, 2, 3);
    println!("x: {}, y: {}, z: {}", x, y, z);

    // function
    fn foo(x: i32) -> i32 {
        x
    }

    let x = foo(5);
    println!("{}", x);

    // multiple patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // multiple patter expressions
    // match guard
    let x = Some(5);
    let y = 3;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    // match guard with multiple values
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // ranges
    let x = 1;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something"),
    }

    // destructuring
    let p = Point { x: 0, y: 7 };

    println!("point coordinates: ({}, {})", p.x, p.y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis at ({}, {})", x, y),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }

    let msg = MessageEnum::ChangeColor(ColorEnum::Hsv(0, 160, 255));

    match msg {
        MessageEnum::ChangeColor(ColorEnum::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        MessageEnum::ChangeColor(ColorEnum::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => (),
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet: {}, inches: {}, x: {}, y: {}", feet, inches, x, y);

    // ignoring values
    let (x, _, z) = (1, 2, 3);
    println!("x: {}, z: {}", x, z);

    fn bar(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    bar(3, 4); // This code only uses the y parameter: 4

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    } // Some numbers: 2, 32

    match numbers {
        (first, ..) => {
            println!("First number is {}", first);
        }
    }

    // bindings
    let msg = MessageBind::Hello { id: 5 };

    match msg {
        MessageBind::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        MessageBind::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        MessageBind::Hello { id } => println!("Some other id: {id}"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[allow(dead_code)]
enum ColorEnum {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[allow(dead_code)]
enum MessageEnum {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(ColorEnum),
}

#[allow(dead_code)]
enum MessageBind {
    Hello { id: i32 },
}

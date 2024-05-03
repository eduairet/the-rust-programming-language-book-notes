fn main() {
    // Vectors

    let v1 = vec![1, 2, 3];
    for i in &v1 {
        println!("{}", i);
    }

    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    for i in &v2 {
        println!("{}", i);
    }
    println!("v2[0] = {}", v2[0]);
    println!("v2.get(1) = {}", v2.get(1).unwrap());

    match v2.get(2) {
        Some(third) => println!("v2[2] = {}", third),
        None => println!("There is no third element."),
    }

    let mut v_mut = vec![100, 32, 57];
    for i in &mut v_mut {
        *i += 50;
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        match i {
            SpreadsheetCell::Int(value) => println!("Int: {}", value),
            SpreadsheetCell::Text(value) => println!("Text: {}", value),
            SpreadsheetCell::Float(value) => println!("Float: {}", value),
        }
    }

    // Strings
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

use std::collections::HashMap;

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

    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);

    let s = String::from("👋👋👋👋");
    println!("{}", s);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let s = "Здравствуйте".to_string();
    println!("{}", s);
    println!("{}", &s[0..4]);
    for c in s.chars() {
        println!("{}", c);
    }
    for b in s.bytes() {
        println!("{}", b);
    }

    // Hash Maps

    let mut scores = HashMap::new(); // Creates a new empty hash map.

    scores.insert(String::from("Blue"), 10); // Inserts a key-value pair into the hash map.
    scores.insert(String::from("Yellow"), 50); // Inserts another key-value pair into the hash map.

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // Retrieves a value from the hash map.
                                                              // The get method returns an Option<&V>. If get finds a value for the given key, it returns Some(&value), and if it doesn’t find the value, it returns None.
    println!("The score of the Blue team is: {:?}", score);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{:?}", field_name); // This line will not compile because field_name has been moved into the hash map.

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50); // Inserts a value if the key has no value.
    scores.entry(String::from("Blue")).or_insert(50); // Doesn't insert a value because the key already has a value.

    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    // Exercises

    let mut v001 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("The median is: {}", median(&mut v001));

    let s001 = "hello world wonderful world";
    println!("{}", pig_latin(s001));

    add_employee_to_company("Add Sally to Engineering");
    add_employee_to_company("Add Amir to Sales");
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn median(v: &mut Vec<i32>) -> f64 {
    /*
    Given a list of integers,
    use a vector and return the median
    (when sorted, the value in the middle position)
    and mode (the value that occurs most often; a hash map will be helpful here)
    of the list.
     */
    v.sort();
    let len = v.len();
    if len % 2 == 0 {
        (v[len / 2] + v[len / 2 - 1]) as f64 / 2.0
    } else {
        v[len / 2] as f64
    }
}

fn pig_latin(s: &str) -> String {
    /*
    Convert strings to pig latin.
    The first consonant of each word is moved
    to the end of the word and ay is added,
    so first becomes irst-fay.
    Words that start with a vowel have hay added
    to the end instead (apple becomes apple-hay).
    Keep in mind the details about UTF-8 encoding!
    */

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut result = String::new();
    for word in s.split_whitespace() {
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();
        if vowels.contains(&first_char) {
            result.push_str(&format!("{}-hay ", word));
        } else {
            result.push_str(&format!("{}-{}ay ", chars.collect::<String>(), first_char));
        }
    }

    result
}

fn add_employee_to_company(input: &str) {
    /*
    Using a hash map and vectors,
    create a text interface to allow a user
    to add employee names to a department
    in a company; for example,
    “Add Sally to Engineering” or “Add Amir to Sales.”
    Then let the user retrieve a list of all people in a department
    or all people in the company by department,
    sorted alphabetically.
    */

    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    let words: Vec<&str> = input.split_whitespace().collect();
    let name = words[1].to_string();
    let department = words[3].to_string();

    let employees = company.entry(department).or_insert(Vec::new());
    employees.push(name);

    for (department, employees) in &company {
        println!("{}: {:?}", department, employees);
    }
}

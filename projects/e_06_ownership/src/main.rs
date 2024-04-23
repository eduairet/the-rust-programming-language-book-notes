fn main() {
    // Ownership

    // Variable Scope
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // string literal valid for the entire scope
        println!("{}", s);
    } // s goes out of scope here

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // Memory and Allocation
    {
        let mut n1 = 5;
        let mut n2 = n1;
        n1 -= 1;
        n2 += 1;
        // n1 and n2 are integers, so they are stored on the stack and we can copy the values
        println!("n1: {}, n2: {}", n1, n2);

        let mut s1 = String::from("hello");
        let s2 = s1.clone();
        s1.push_str(", world!"); // Error: value borrowed here after move
        println!("{s1}\n{s2}");
    }

    // Ownership and Functions & Return Values and Scope
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // Error: value borrowed here after move

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // println!("{}", s2); // Error: value borrowed here after move

    // References and Borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len); // s1 is still valid

    let mut s = String::from("hello");
    let r1 = &s;
    println!("{}", r1); // r1 goes out of scope here

    let r2 = &mut s; // no problem
    println!("{}", r2);

    // The Slice Type
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value hello
                               //s.clear(); // this will cause an error
    println!("The first word is: {}", word); // The first word is hello

    // String Slices
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let hello = &s[..5]; // start is 0
    let world = &s[6..]; // end is the length of the string
    println!("{} {}", hello, world);

    let s = "hello world";
    let hello = &s[..]; // start is 0 and end is the length of the string
    println!("{}", hello);

    // Other Slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{:?}", slice);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> &str {
    //let bytes = s.as_bytes();

    //for (i, &item) in bytes.iter().enumerate() {
    //     if item == b' ' {
    //         return i;
    //     }
    // }

    // s.len()

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

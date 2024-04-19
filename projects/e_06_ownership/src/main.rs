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

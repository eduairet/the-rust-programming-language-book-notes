use std::ops::{Deref, DerefMut};

fn main() {
    let b = Box::new(5); // b is a smart pointer and it is stored on the heap
    println!("b = {}", b);
    // b is deallocated here
    // This is just a simple example of how smart pointers work
    // but is not a good example of when to use them

    // A good use case for lists is for recursive data structures
    let list = LinkedList::Cons(
        1,
        Box::new(LinkedList::Cons(
            2,
            Box::new(LinkedList::Cons(3, Box::new(LinkedList::Nil))),
        )),
    );
    println!("{:#?}", list);

    // Following the pointer to the value
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y); // Dereferencing the pointer
                       // without the * we would get an error since we're not comparing the actual value

    // Creating a smart pointer
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Creating a custom smart pointer
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Deref coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // Rust automatically dereferences the MyBox to a string
               // Without Deref coercion we would have to write hello(&(*m)[..])

    // DerefMut trait
    let mut m = MyBox::new(String::from("Rust"));
    hello_mut(&mut m);
}

#[derive(Debug)]
#[allow(dead_code)]
enum LinkedList {
    Cons(i32, Box<LinkedList>),
    Nil,
}

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
// Implementing the Deref trait to allow dereferencing
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // Returning a reference to the value inside the MyBox
        &self.0
    }
}
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        // Returning a mutable reference to the value inside the MyBox
        &mut self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn hello_mut(name: &mut MyBox<String>) {
    name.push_str("!");
    println!("Hello, {:#?}!", name);
}

use std::{
    cell::RefCell,
    mem::drop,
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
};

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

    // The Drop trait
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // Dropping a value before it goes out of scope
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c); // Dropping the value before it goes out of scope
    println!("CustomSmartPointer dropped before the end of main.");

    // Shared pointers
    let a = Rc::new(LinkedListMultiple::Cons(
        5,
        Rc::new(LinkedListMultiple::Cons(
            10,
            Rc::new(LinkedListMultiple::Nil),
        )),
    ));
    let _b = LinkedListMultiple::Cons(3, Rc::clone(&a));
    let _c = LinkedListMultiple::Cons(4, Rc::clone(&a)); // Cloning the reference to a
    println!("{:#?}", a);
    println!("Reference count: {}", Rc::strong_count(&a));

    let value = Rc::new(RefCell::new(5));
    let value2 = Rc::clone(&value);

    *value.borrow_mut() += 10;
    *value2.borrow_mut() += 20;

    println!("value = {:?}", value.borrow()); // value = RefCell { value: 35 }

    // Multiple owners of mutable data
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(ListMultiple::Cons(
        // Creating a list with multiple owners of mutable data
        Rc::clone(&value),
        Rc::new(ListMultiple::Nil),
    ));

    // Creating two more references to the list
    let b = ListMultiple::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = ListMultiple::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // Modifying the value
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // Reference cycle
    let a = Rc::new(ListRefCycle::Cons(
        5,
        RefCell::new(Rc::new(ListRefCycle::Nil)),
    ));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(ListRefCycle::Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    // println!("a next item = {:?}", a.tail());

    // Preventing reference cycles: using Weak
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
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

// The Drop trait
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // When the object goes out of scope this is triggered
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// Shared pointers
#[derive(Debug)]
#[allow(dead_code)]
enum LinkedListMultiple {
    Cons(i32, Rc<LinkedListMultiple>),
    Nil,
}

// Mock implementation of RefCell
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent: You're at 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You're at 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // Implementing a mock Messenger to test the LimitTracker
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // Creating a new MockMessenger
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// Multiple owners of mutable data
#[derive(Debug)]
#[allow(dead_code)]
enum ListMultiple {
    Cons(Rc<RefCell<i32>>, Rc<ListMultiple>),
    Nil,
}

// Reference cycle
#[derive(Debug)]
#[allow(dead_code)]
enum ListRefCycle {
    Cons(i32, RefCell<Rc<ListRefCycle>>),
    Nil,
}

impl ListRefCycle {
    fn tail(&self) -> Option<&RefCell<Rc<ListRefCycle>>> {
        match self {
            ListRefCycle::Cons(_, item) => Some(item),
            ListRefCycle::Nil => None,
        }
    }
}

// Preventing reference cycles: using Weak
#[derive(Debug)]
#[allow(dead_code)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

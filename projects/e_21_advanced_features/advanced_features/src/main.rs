use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use std::{fmt, ops::Add, slice};

fn main() {
    // Dereferencing a raw pointer
    let mut num = 5;

    // We can create raw pointers in safe code using as
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        // We can only dereference raw pointers in unsafe code
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe {
        *r2 = 10;
    }

    println!("num is: {}", num);

    let address = 0x012345usize;
    let _r = address as *const i32;

    // Calling an unsafe function
    unsafe {
        dangerous();
    }

    // Creating a safe abstraction over unsafe code
    let mut values = [1, 2, 3, 4, 5];
    let (left, _right) = split_at_mut(&mut values, 2);

    println!("{:?}", left);

    // Using extern functions to call external code
    let input = -3;
    let output = unsafe { abs(input) };
    println!("Absolute value of {} is: {}", input, output);

    // Accessing or modifying mutable static variables
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // Accessing fields of unions
    let mut iof = IntOrFloat { i: 123 };

    unsafe {
        println!("iof.i: {}", iof.i);
    }

    iof.i = 234;

    unsafe {
        println!("iof.i: {}", iof.i);
    }

    // Operation overloading
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // Disambiguating methods with the same name
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);

    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    // We need to use <Dog as Animal> to disambiguate the call to baby_name

    // Supertraits
    let p = SupertraitPoint { x: 1, y: 2 };
    p.outline_print();

    // The new type pattern
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    let mm = Millimeters(1000);
    let m = Meters(1);

    let mm_plus_m = mm + m;

    println!("mm_plus_m = {:?}", mm_plus_m);

    // Type aliases for type synonyms
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // Returning closures
    let f = returns_closure();
    let answer = f(5);

    println!("The answer is: {}", answer);

    // Macros
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    Pancakes::hello_macro();

    // The never type that never returns
    bar();
}

unsafe fn dangerous() {
    // This code block is unsafe so we can do anything here without using unsafe keyword
    println!("This is a dangerous function");
}

// Splitting a slice using unsafe code and wrapping it in a safe function+

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Using extern functions to call external code
extern "C" {
    fn abs(input: i32) -> i32;
}

// Accessing or modifying mutable static variables
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// Defining an unsafe trait
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

// Accessing fields of unions
#[allow(dead_code)]
union IntOrFloat {
    i: i32,
    f: f32,
}

// Operation overloading
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Disambiguating methods with the same name
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Supertraits
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct SupertraitPoint {
    x: i32,
    y: i32,
}

impl OutlinePrint for SupertraitPoint {}

impl fmt::Display for SupertraitPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// The newtype pattern
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// Newtype pattern is used to guarantee type safety and abstraction
#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Type aliases fot type synonyms
type Kilometers = i32;

// The never type that never returns
fn bar() -> ! {
    panic!("This function never returns!");
}

// Returning closures
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// Macros
#[macro_export]
macro_rules! vec {
    // The $() syntax is used to indicate that the pattern expects an argument
    // The * syntax is used to indicate that the pattern expects zero or more arguments
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// Custom derive
#[derive(HelloMacro)]
struct Pancakes;

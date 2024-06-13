use std::slice;

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
}

unsafe fn dangerous() {
    // This code block is unsafe so we can do anything here without using unsafe keyword
    println!("This is a dangerous function");
}

// Splitting a slice using unsafe code and wrapping it in a safe function
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

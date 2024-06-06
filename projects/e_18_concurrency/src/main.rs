use std::{thread, time::Duration};

fn main() {
    // Creating a new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    // Wait for the spawned thread to finish
    handle.join().unwrap();
    // It does matter where you put this line,
    // if it is before the loop,
    // the main thread will wait for the spawned thread
    // to finish before starting the loop

    // Use move closure to move data into the spawned thread
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

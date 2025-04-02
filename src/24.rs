// Rust code example demonstrating thread safety and synchronization using locks.
// Example of safe concurrent modification operations using mutexes with lock propagation.

use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let mut counter = 0;

    // Create a lock to ensure that the critical section is protected
    let _lock = Mutex::new(counter);

    for i in 1..=5 {
        println!("Value of counter: {}", counter);
        thread::sleep(std::time::Duration::from_secs(1)); // Simulate some work

        // Acquire and release the lock to safely modify the value
        let _value = _lock.lock().unwrap();

        if i % 2 == 0 {
            counter += 1;
            println!("Incrementing counter by {}", i);
        }
    }

    println!("Final value of counter: {}", counter);
}


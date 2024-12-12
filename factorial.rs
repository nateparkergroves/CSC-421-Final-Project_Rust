use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub fn factorial() {
    let start = Instant::now();
    let n = 7; // Number to calculate factorial

    // Create a shared variable to store the result
    let result = Arc::new(Mutex::new(1));

    // Create a vector to hold the thread handles
    let mut handles = vec![];

    // Spawn threads to calculate factorial
    for i in 1..=n {
        let result = Arc::clone(&result);
        let handle = thread::spawn(move || {
            let mut result = result.lock().unwrap();
            *result *= i;
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the factorial result
    let result = result.lock().unwrap();
    let duration = start.elapsed();

    println!("Factorial of {} is: {}", n, *result);
    println!("This factorial took {:?} time to complete", duration);

}
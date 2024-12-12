use std::thread;
use std::thread::JoinHandle;

const NTHREADS: usize = 20;

pub fn thread_example() {
    // 1. Create a vector to hold thread handles
    let mut children: Vec<JoinHandle<()>> = vec![];

    // 2. Spawn NTHREADS threads
    for i in 0..NTHREADS {
        let i = i;
        children.push(thread::spawn(move || {
            println!("This is thread number {}", i);
        }));
    }

    // 5. Wait for all threads to finish
    for child in children {
        let _ = child.join().unwrap();
    }
}
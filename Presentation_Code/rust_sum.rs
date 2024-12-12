use std::thread;
use std::time::Instant;

fn main() {
    let ranges = vec![
        (1, 50_000_000),
        (50_000_001, 100_000_000),
        (100_000_001, 150_000_000),
        (150_000_001, 200_000_000),
    ];

    let start_time = Instant::now(); // Start the timer

    let mut handles = vec![];

    // Start threads for each range
    for (start, end) in ranges {
        let handle = thread::spawn(move || {
            (start..=end).fold(0u64, |acc, x| acc + x)
        });
        handles.push(handle);
    }

    let mut total_sum = 0;

    // Collect results from all threads
    for handle in handles {
        total_sum += handle.join().unwrap();
    }

    let duration = start_time.elapsed(); // Calculate elapsed time

    println!("Total sum: {}", total_sum);
    println!("Time taken: {:.2?}", duration);
}

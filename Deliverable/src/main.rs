use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn main() {

    let start_time = Instant::now();

    let text = fs::read_to_string("large_text.txt").expect("Failed to read file");

    // Split the text into chunks as owned Strings
    let chunks: Vec<String> = text.split('\n').map(|s| s.to_string()).collect();

    // Shared data structure: a HashMap protected by a Mutex inside an Arc
    let word_freq = Arc::new(Mutex::new(HashMap::new()));

    let mut handles = vec![];

    for chunk in chunks {
        let word_freq = Arc::clone(&word_freq);

        // Spawn a thread to process the chunk
        let handle = thread::spawn(move || {
            let mut local_freq = HashMap::new();

            // Count word frequencies in the current chunk
            for word in chunk.split_whitespace() {
                *local_freq.entry(word.to_string()).or_insert(0) += 1;
            }

            // Safely update the shared HashMap
            let mut global_freq = word_freq.lock().unwrap();
            for (word, count) in local_freq {
                *global_freq.entry(word).or_insert(0) += count;
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start_time.elapsed();


    println!("Time taken: {:.2?}", duration);

    let word_freq = word_freq.lock().unwrap();
    for (word, count) in word_freq.iter() {
        println!("{}: {}", word, count);
    }

}

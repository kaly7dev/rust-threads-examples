// examples/sharding.rs
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    const NUM_SHARDS: usize = 4;
    const NUM_THREADS: usize = 16;
    const INCREMENTS_PER_THREAD: usize = 100_000;

    // Create shards: Vec<Arc<Mutex<usize>>>
    let shards: Vec<Arc<Mutex<usize>>> = (0..NUM_SHARDS).map(|_| Arc::new(Mutex::new(0))).collect();

    let mut handles = vec![];

    for i in 0..NUM_THREADS {
        let shard = Arc::clone(&shards[i % NUM_SHARDS]); // Assign shard round-robin
        let handle = thread::spawn(move || {
            for _ in 0..INCREMENTS_PER_THREAD {
                let mut count = shard.lock().unwrap();
                *count += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Sum all shards
    let total: usize = shards.iter().map(|s| *s.lock().unwrap()).sum();
    println!("Total: {}", total);
}
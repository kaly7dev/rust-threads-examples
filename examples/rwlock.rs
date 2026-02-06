use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    let mut handles = vec![];

    // 8 readers
    for _ in 0..8 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let read_guard = data.read().unwrap();
            println!("Read: sum = {}", read_guard.iter().sum::<i32>());
        });
        handles.push(handle);
    }

    // 2 writers
    for i in 4..6 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut write_guard = data.write().unwrap();
            write_guard.push(i);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Final data: {:?}", *data.read().unwrap());
}
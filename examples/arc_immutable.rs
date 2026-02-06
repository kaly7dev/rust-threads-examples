use std::sync::Arc;
use std::thread;

fn main() {
    let config = Arc::new(vec![String::from("host=db.example.com"); 1000]);

    let config1 = Arc::clone(&config);
    let handle1 = thread::spawn(move || {
        println!("Worker 1 sees {} entries", config1.len());
    });

    let config2 = Arc::clone(&config);
    let handle2 = thread::spawn(move || {
        println!("Worker 2 sees {} entries", config2.len());
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    // Original Arc still alive
    println!("Main still has config with {} entries", config.len());
}
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread: {}", i);
        }
    });

    for i in 1..=4 {
        println!("Main: {}", i);
    }

    handle.join().unwrap(); // Wait for the thread to finish
}
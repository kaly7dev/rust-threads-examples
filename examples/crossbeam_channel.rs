use crossbeam::channel;
use std::thread;

fn main() {
    let (tx, rx) = channel::unbounded();

    for i in 0..4 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(format!("Hello from thread {i}")).unwrap();
        });
    }

    drop(tx); // Close all senders

    for msg in rx {
        println!("{msg}");
    }
}
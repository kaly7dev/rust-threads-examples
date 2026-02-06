use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i).unwrap();
        }
        // Sender dropped → receiver sees end
    });

    for received in rx {
        println!("Got: {received}");
    }
}
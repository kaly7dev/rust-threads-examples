use std::thread;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let handle = thread::spawn(move || {
        let sum: i32 = numbers.iter().sum();
        println!("Thread computed sum: {sum}");
    });

    // numbers is moved, so we can't use it here anymore
    handle.join().unwrap();
}
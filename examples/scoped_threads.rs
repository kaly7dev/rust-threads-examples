use std::thread;

fn main() {
    let data = vec![10, 20, 30];

    thread::scope(|s| {
        s.spawn(|| {
            println!("Borrowed in thread 1: {}", data[0]);
        });

        s.spawn(|| {
            let sum: i32 = data.iter().sum();
            println!("Borrowed in thread 2: sum = {sum}");
        });
    });

    // data is still usable here
    println!("Back in main: {:?}", data);
}
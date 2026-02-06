use rayon::prelude::*;

fn main() {
    let numbers: Vec<_> = (0..1_000_000).collect();

    let sum: u64 = numbers.par_iter().map(|&x| x as u64).sum();

    println!("Parallel sum: {sum}");
}
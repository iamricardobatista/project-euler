use std::usize;

use smallest_multiple::compute;

fn main() {
    let numbers: Vec<usize> = (1..=20).collect();
    let result = compute(&numbers);
    println!("smallest positive number that is evenly divisible is {result}");
}

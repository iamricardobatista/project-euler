use even_fibonacci_numbers::FibonacciSequence;

fn main() {
    let fibonacci_sequence = FibonacciSequence::default();

    let result: usize = fibonacci_sequence
        .take_while(|x| x.lt(&4000000))
        .filter(|x| x % 2 == 0)
        .sum();

    println!("The sum of all first even terms not exceding 4000000 of the fibonacci sequence is {result}");
}

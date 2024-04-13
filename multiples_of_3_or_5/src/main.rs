use multiples_of_3_or_5::sum_mutiples;

fn main() {
    let multiples = vec![3, 5];
    let upper_limit = 1000;

    println!("Result: {}", sum_mutiples(multiples, upper_limit));
}

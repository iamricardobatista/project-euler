use largest_prime_factor::PrimeFactors;

fn main() {
    let number = 600851475143;
    let factors = PrimeFactors::new(number);
    let max_factor = factors.max().unwrap_or(0);
    println!("The max factor is: {max_factor}");
}

use st_prime::Primes;

fn main() {
    let mut primes = Primes::default();
    let result = primes.nth(1000000 - 1).unwrap();
    println!("The 10001sh prime number is {:?}", result);
}

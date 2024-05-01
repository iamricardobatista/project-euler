use cucumber::{given, then, World};
use st_prime::Primes;

#[derive(Debug, Default, World)]
struct StPrimeWorld {
    primes: Primes,
}

#[given(expr = "a prime numbers iterator")]
fn prime_numbers_iterator(world: &mut StPrimeWorld) {
    world.primes = Primes::default();
}

#[then(expr = "the {int} th prime number is {int}")]
fn n_prime_number(world: &mut StPrimeWorld, index: usize, expected: usize) {
    let result = world.primes.nth(index - 1).unwrap_or(0);
    assert!(result == expected, "Result: {result}, expected: {expected}");
}

#[tokio::main]
async fn main() {
    StPrimeWorld::cucumber()
        .fail_on_skipped()
        .run("tests/features/10001st_prime.feature")
        .await;
}

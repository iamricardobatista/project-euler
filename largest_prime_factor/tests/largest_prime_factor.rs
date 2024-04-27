use cucumber::{gherkin::Step, given, then, World};

#[derive(Default, Debug, World)]
struct LargestPrimeFactorWorld {
    prime_factors: largest_prime_factor::PrimeFactors,
}

#[given(expr = "the prime factors for number {int}")]
fn prime_factors_for_number(world: &mut LargestPrimeFactorWorld, number: usize) {
    world.prime_factors = largest_prime_factor::PrimeFactors::new(number);
}

#[then(expr = "its prime factors are")]
fn its_prime_factors(world: &mut LargestPrimeFactorWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        let expected: Vec<usize> = table
            .rows
            .iter()
            .skip(1)
            .map(|x| x[0].parse().unwrap())
            .collect();

        let result: Vec<usize> = world.prime_factors.collect();

        assert!(
            expected.eq(&result),
            "Result: {:?}, Expected: {:?}",
            result,
            expected
        );
    };
}

#[tokio::main]
async fn main() {
    LargestPrimeFactorWorld::cucumber()
        .fail_on_skipped()
        .run("tests/features/largest_prime_factor.feature")
        .await;
}

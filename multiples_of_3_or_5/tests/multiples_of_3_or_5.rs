use cucumber::{gherkin::Step, given, then, World};
use multiples_of_3_or_5::sum_mutiples;

#[derive(Debug, Default, World)]
struct MultiplesWorld {
    natural_number: usize,
}

#[given(expr = "the positive natural number {int}")]
fn positive_natural_number(world: &mut MultiplesWorld, natural_number: usize) {
    world.natural_number = natural_number;
}

#[then(expr = "the sum of all multiples of natural positive numbers bellow that number is {int}")]
fn sum_of_all_multiples_of(world: &mut MultiplesWorld, step: &Step, expected: usize) {
    if let Some(table) = step.table.as_ref() {
        let multiples: Vec<usize> = table
            .rows
            .iter()
            .skip(1)
            .map(|x| x[0].parse().unwrap())
            .collect();

        let result = sum_mutiples(multiples, world.natural_number);

        assert!(
            result == expected,
            "result: {}, expected: {}",
            result,
            expected
        );
    };
}

#[tokio::main]
async fn main() {
    MultiplesWorld::cucumber()
        .fail_on_skipped()
        .run("tests/features/multiples.feature")
        .await;
}

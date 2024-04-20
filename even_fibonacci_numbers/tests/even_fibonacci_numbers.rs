use cucumber::{gherkin::Step, given, then, World};
use even_fibonacci_numbers::FibonacciSequence;

#[derive(Debug, Default, World)]
struct FibonacciWorld {
    sequence: FibonacciSequence,
}

#[given(expr = "a fibonacci sequence")]
fn a_fibonacci_sequence(world: &mut FibonacciWorld) {
    world.sequence = FibonacciSequence::default();
}

#[then(expr = "the first terms are")]
fn the_first_terms(world: &mut FibonacciWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        let multiples: Vec<usize> = table
            .rows
            .iter()
            .skip(1)
            .map(|x| x[0].parse().unwrap())
            .collect();

        for expected_term in multiples {
            let current_term = world.sequence.next().unwrap();

            assert!(
                expected_term == current_term,
                "Expected term: {expected_term}, Current term: {current_term}"
            )
        }
    };
}

#[tokio::main]
async fn main() {
    FibonacciWorld::cucumber()
        .fail_on_skipped()
        .run("tests/features/fibonacci.feature")
        .await;
}

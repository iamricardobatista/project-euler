use std::usize;

use cucumber::{given, then, World};
use smallest_multiple::compute;

#[derive(Default, Debug, World)]
struct SmallestMultipleWorld {
    numbers: Vec<usize>,
}

#[given(expr = "a set of numbers from {int} to {int}")]
fn a_set_of_mumbers(world: &mut SmallestMultipleWorld, start: usize, finish: usize) {
    world.numbers = (start..=finish).collect();
}

#[then(
    expr = "the smallest number that can be divided by all numbers of the set without remainder is {int}"
)]
fn smallest_evenly_divider(world: &mut SmallestMultipleWorld, expected: usize) {
    let result = compute(&world.numbers);
    assert!(result == expected, "Result: {result}, expected: {expected}");
}

#[tokio::main]
async fn main() {
    SmallestMultipleWorld::cucumber()
        .fail_on_skipped()
        .run("tests/features/smallest_multiple.feature")
        .await;
}

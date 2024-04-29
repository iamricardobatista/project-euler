use std::usize;

use cucumber::{given, then, World};
use sum_square_difference::{square_sum, sum_square};

#[derive(Debug, Default, World)]
struct SumSquareDifferenceWorld {
    natural_number: usize,
    sum_squares: usize,
    squares_sum: usize,
}

#[given(expr = "a natural number {int}")]
fn a_natural_number(world: &mut SumSquareDifferenceWorld, number: usize) {
    world.natural_number = number;
}

#[then(expr = "the sum of squares is {int}")]
fn the_sum_of_squares(world: &mut SumSquareDifferenceWorld, expected: usize) {
    world.sum_squares = sum_square(world.natural_number);
    assert!(
        world.sum_squares == expected,
        "Result: {0}, expected: {expected}",
        world.sum_squares
    );
}

#[then(expr = "the difference between the sum of squares and squares of the sum is {int}")]
fn sum_of_squares_less_squares_of_the_sum(world: &mut SumSquareDifferenceWorld, expected: usize) {
    let result = world.squares_sum - world.sum_squares;
    assert!(result == expected, "Result: {result}, expected: {expected}");
}

#[then(expr = "the squares of the sum is {int}")]
fn the_squares_of_the_sum(world: &mut SumSquareDifferenceWorld, expected: usize) {
    world.squares_sum = square_sum(world.natural_number);
    assert!(
        world.squares_sum == expected,
        "Result: {0}, expected: {expected}",
        world.squares_sum
    );
}

#[tokio::main]
async fn main() {
    SumSquareDifferenceWorld::cucumber()
        .fail_on_skipped()
        .run("tests/features/sum_square_difference.feature")
        .await;
}

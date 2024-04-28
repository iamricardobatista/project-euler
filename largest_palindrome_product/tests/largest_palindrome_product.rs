use cucumber::{given, then, World};
use largest_palindrome_product::LargestPalindromeProduct;

#[derive(Debug, Default, World)]
struct LargestPalinDromeProductWorld {
    largest_palindrome_product: LargestPalindromeProduct,
}

#[given(expr = "a number of digits {int}")]
fn number_of_digits(world: &mut LargestPalinDromeProductWorld, digits_number: u8) {
    world.largest_palindrome_product = LargestPalindromeProduct::new(digits_number);
}

#[then(expr = "the largest palindrome from a product of that number of digits is {int}")]
fn largest_palindrome_product(world: &mut LargestPalinDromeProductWorld, expected: usize) {
    let result = world.largest_palindrome_product.compute();
    assert!(result == expected, "Result {result}, Expected: {expected}");
}

#[tokio::main]
async fn main() {
    LargestPalinDromeProductWorld::cucumber()
        .fail_on_skipped()
        .run("tests/features/largest_palindrome_product.feature")
        .await;
}

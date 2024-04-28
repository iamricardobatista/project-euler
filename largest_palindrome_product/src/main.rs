use largest_palindrome_product::LargestPalindromeProduct;

fn main() {
    let largest_palindrome_product = LargestPalindromeProduct::new(3);
    let result = largest_palindrome_product.compute();
    println!("Largest palindrome product of a 3-digit numbers is {result}");
}

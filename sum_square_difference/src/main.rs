use sum_square_difference::{square_sum, sum_square};

fn main() {
    let number = 100;
    let result = square_sum(number) - sum_square(number);
    println!("the difference between the sum of squares and squares of the sum is {result}");
}

pub fn sum_square(number: usize) -> usize {
    (1..=number).fold(0, |acc, x| acc + x.pow(2))
}
pub fn square_sum(number: usize) -> usize {
    (1..=number).sum::<usize>().pow(2)
}

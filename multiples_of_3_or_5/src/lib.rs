/// Given a list of positive natural numbers sums all multiple values of those naturals under an
/// upper limit
///
/// # Examples
///
/// ```
/// use multiples_of_3_or_5::sum_mutiples;
///
/// let multiples = vec![3, 5];
/// let upper_limit = 10;
/// let expected = 23;
///
/// let result = sum_mutiples(multiples, upper_limit);
/// assert!(result == expected);
///
/// ```
pub fn sum_mutiples(naturals: Vec<usize>, upper_limit: usize) -> usize {
    let mut accumulator: usize = 0;

    for iteration in 1..upper_limit {
        for natural in &naturals {
            if iteration % natural == 0 {
                accumulator += iteration;
                break; // so we don't have duplicates
            }
        }
    }

    accumulator
}

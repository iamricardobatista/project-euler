use std::usize;

pub fn compute(numbers: &[usize]) -> usize {
    let mut n = 1;
    loop {
        if numbers.iter().all(|&x| n % x == 0) {
            return n;
        }
        n += 1;
    }
}

#[derive(Debug)]
pub struct FibonacciSequence {
    first: usize,
    second: usize,
}

impl Default for FibonacciSequence {
    fn default() -> Self {
        Self {
            first: 1,
            second: 2,
        }
    }
}

impl Iterator for FibonacciSequence {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.first;

        (self.first, self.second) = (self.second, self.first + self.second);

        Some(result)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PrimeFactors {
    current: usize,
}

impl PrimeFactors {
    pub fn new(number: usize) -> Self {
        Self { current: number }
    }
}

impl Iterator for PrimeFactors {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        for n in 2..=self.current {
            if self.current % n != 0 {
                continue;
            }

            self.current /= n;
            return Some(n);
        }
        None
    }
}

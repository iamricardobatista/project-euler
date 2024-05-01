#[derive(Debug, Default)]
pub struct Primes {
    primes: Vec<usize>,
}

impl Iterator for Primes {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.primes.is_empty() {
            self.primes.push(2);
            println!("Current prime: 2");
            return Some(2);
        }

        let mut current = *self.primes.last().unwrap();

        loop {
            current += 1;
            if !self.primes.iter().any(|x| current % x == 0) {
                self.primes.push(current);
                println!("Current prime: {current}");
                return Some(current);
            }
        }
    }
}

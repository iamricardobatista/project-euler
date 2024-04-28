use std::{u8, usize};

#[derive(Default, Debug)]
pub struct LargestPalindromeProduct {
    digits_number: u8,
}

pub fn is_palindrome(number: usize) -> bool {
    let number_string = number.to_string();
    let reversed = number_string.clone().chars().rev().collect::<String>();
    number_string == reversed
}

impl LargestPalindromeProduct {
    pub fn new(digits_number: u8) -> Self {
        Self { digits_number }
    }

    fn build_product_upper_limit(&self) -> usize {
        let mut limit = String::new();

        for _ in 0..self.digits_number {
            limit.push('9');
        }

        limit.parse().unwrap()
    }

    fn build_product_lower_bound(&self) -> usize {
        let mut limit = String::new();

        for _ in 1..self.digits_number {
            limit.push('9');
        }

        limit.parse::<usize>().unwrap() + 1
    }

    pub fn compute(&self) -> usize {
        let upper_limit = self.build_product_upper_limit();
        let lower_limit = self.build_product_lower_bound();

        let limit = upper_limit * upper_limit;

        for n in (1..=limit).rev() {
            for i in (lower_limit..=upper_limit).rev() {
                let remaining = n % i;
                let result = n / i;

                if remaining != 0 {
                    continue;
                }

                if result.ge(&lower_limit) && result.le(&upper_limit) && is_palindrome(n) {
                    return n;
                }
            }
        }

        limit
    }
}

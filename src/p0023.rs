//! [Problem 23](https://projecteuler.net/problem=23)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2023))

use super::common::get_prime_factor_sums;
use std::collections::HashMap;

// TODO 固定値を与えられているのはなんか気持ち悪い
pub const MAX_SUM: usize = 28123;

pub fn solve(input: usize) -> i64 {
    // エラトステネスの篩の要領でやれると思う
    let abundant_numbers: HashMap<_, _> = get_prime_factor_sums(input as u64)
        .into_iter()
        .filter(|(i, f)| i < f)
        .collect();
    (1..=input as u64)
        .map(|i| {
            match abundant_numbers
                .iter()
                .find(|(&n, _)| i > n && abundant_numbers.contains_key(&(i - n)))
            {
                Some(_) => 0,
                _ => i,
            }
        })
        .sum::<u64>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(2766, solve(100));
    }
}

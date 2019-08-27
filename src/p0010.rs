//! [Problem 10](https://projecteuler.net/problem=10)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2010))

use super::common::Primes;

pub struct Solver;

const MAX: u64 = 2_000_000;

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(MAX)
    }
}

fn solve(input: u64) -> u64 {
    Primes::new().take_while(|p| p <= &input).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_prime_numbers() {
        let ts = vec![
            (1, 0),
            (2, 2),
            (3, 5),
            (4, 5),
            (5, 10),
            (6, 10),
            (7, 17),
            (8, 17),
            (9, 17),
            (10, 17),
            (11, 28),
        ];
        for (input, expected) in ts {
            assert_eq!(expected, solve(input));
        }
    }
}

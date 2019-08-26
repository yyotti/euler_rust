//! [Problem 7](https://projecteuler.net/problem=7)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%207))

use super::common::Primes;

pub struct Solver;

const NUM: u64 = 10001;

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(NUM)
    }
}

fn solve(input: u64) -> u64 {
    Primes::new().skip((input as usize) - 1).next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_nth_prime_number() {
        let ts = vec![
            (1, 2),
            (2, 3),
            (3, 5),
            (4, 7),
            (5, 11),
            (6, 13),
        ];
        for (input, expected) in ts {
            assert_eq!(expected, solve(input));
        }
    }
}

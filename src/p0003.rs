//! [Problem 3](https://projecteuler.net/problem=3)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%203))

pub struct Solver;

use super::common::Primes;

const NUM: u64 = 600_851_475_143;

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(NUM)
    }
}

fn solve(input: u64) -> u64 {
    if input < 2 {
        return input
    }

    let mut n = input;
    for p in Primes::new() {
        while n % p == 0 {
            n /= p
        }

        if n == 1 {
            return p
        }
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_max_prime_factor() {
        let ts = vec![
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 2),
            (5, 5),
            (6, 3),
            (7, 7),
            (8, 2),
            (9, 3),
            (10, 5),
            (13195, 29),
        ];
        for (input, expected) in ts {
            assert_eq!(expected, solve(input));
        }
    }
}

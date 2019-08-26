//! [Problem 5](https://projecteuler.net/problem=5)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%205))

pub struct Solver;

use super::common::Primes;
use std::collections::HashMap;

const NUM: u64 = 20;

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(NUM)
    }
}

fn solve(input: u64) -> u64 {
    (1..input+1)
        .flat_map(prime_factors)
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.insert(k, *acc.get(&k).unwrap_or(&0).max(&v));
            acc
        })
        .iter()
        .fold(1, |acc, (k, v)| acc * k.pow(*v))
}

fn prime_factors(n: u64) -> HashMap<u64, u32> {
    let mut map = HashMap::new();
    let mut m = n;
    for p in Primes::new().take_while(|p| p <= &n) {
        while m % p == 0 {
            map.insert(p, map.get(&p).unwrap_or(&0) + 1);
            m /= p;
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_smallest_mutiple() {
        assert_eq!(2520, solve(10))
    }

    #[test]
    fn get_prime_factors() {
        let ts = vec![
            (0, HashMap::new()),
            (1, HashMap::new()),
            (2, [(2, 1)].iter().cloned().collect()),
            (3, [(3, 1)].iter().cloned().collect()),
            (4, [(2, 2)].iter().cloned().collect()),
            (5, [(5, 1)].iter().cloned().collect()),
            (6, [(2, 1), (3, 1)].iter().cloned().collect()),
            (7, [(7, 1)].iter().cloned().collect()),
            (8, [(2, 3)].iter().cloned().collect()),
            (9, [(3, 2)].iter().cloned().collect()),
            (10, [(2, 1), (5, 1)].iter().cloned().collect()),
        ];
        for (input, expected) in ts {
            assert_eq!(expected, prime_factors(input));
        }
    }
}

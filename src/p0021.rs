//! [Problem 21](https://projecteuler.net/problem=21)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2021))

use super::common::prime_factors;

pub struct Solver;

const NUM: u64 = 10_000;

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(NUM)
    }
}

fn solve(input: u64) -> u64 {
    // 素直にやる
    (1..input).filter(|&n| has_amicable_number(n)).sum()
}

fn has_amicable_number(n: u64) -> bool {
    let d = sum_factors(n);
    if d < n || d - n == n {
        // d - n == n の場合は完全数なので false
        return false;
    }

    sum_factors(d - n) == d
}

fn sum_factors(n: u64) -> u64 {
    if n < 2 {
        return 0;
    }

    prime_factors(n)
        .iter()
        .fold(1, |acc, (p, e)| acc * (p.pow(e + 1) - 1) / (p - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_factors() {
        assert_eq!(3, sum_factors(2));
        assert_eq!(12, sum_factors(6));
        assert_eq!(217, sum_factors(100));
        assert_eq!(504, sum_factors(220));
        assert_eq!(504, sum_factors(284));
    }

    #[test]
    fn test_has_amicable_number() {
        assert_eq!(false, has_amicable_number(6));
        assert_eq!(false, has_amicable_number(100));
        assert_eq!(true, has_amicable_number(220));
        assert_eq!(true, has_amicable_number(284));
    }

    #[test]
    fn test_solve() {
        // 1000未満の友愛数は (220, 284) のみ
        assert_eq!(504, solve(1000));
    }
}

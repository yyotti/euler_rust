//! [Problem 21](https://projecteuler.net/problem=21)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2021))

use super::common::get_prime_factor_sums;
use super::common::prime_factors;

pub struct Solver;

const NUM: usize = 10_000;

impl super::Solver for Solver {
    fn solve(&self) -> i64 {
        solve(NUM)
    }
}

fn solve(input: usize) -> i64 {
    // いちいち素因数分解してやるのは時間がかかりそうなので、エラトステネス風
    // に約数の和を計算してみる。
    let factors = get_prime_factor_sums(input as u64);

    (1..input as u64)
        .filter_map(|i| {
            if i < 2 {
                return None;
            }

            match factors.get(&i) {
                Some(k) if k != &i && factors.get(k) == Some(&i) => Some(i),
                _ => None,
            }
        })
        .sum::<u64>() as i64
}

#[allow(dead_code)]
fn solve2(input: usize) -> i64 {
    // 素直にやる
    (1..input)
        .filter(|&n| has_amicable_number(n))
        .sum::<usize>() as i64
}

fn has_amicable_number(n: usize) -> bool {
    let d = sum_factors(n);
    if d < n || d - n == n {
        // d - n == n の場合は完全数なので false
        return false;
    }

    let e = sum_factors(d - n);
    e > d - n && e - (d - n) == n
}

fn sum_factors(n: usize) -> usize {
    if n < 2 {
        return 0;
    }

    prime_factors(n as u64).iter().fold(1, |acc, (&p, e)| {
        acc * (p.pow((e + 1) as u32) - 1) as usize / (p as usize - 1)
    })
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
        assert_eq!(true, has_amicable_number(1184));
        assert_eq!(true, has_amicable_number(1210));
    }

    #[test]
    fn test_solve() {
        // 1000未満の友愛数は (220, 284) のみ
        assert_eq!(504, solve(1000), "solve1");
        assert_eq!(504, solve2(1000), "solve2");
    }
}

//! [Problem 21](https://projecteuler.net/problem=21)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2021))

use super::common::prime_factors;
use std::iter::repeat;

pub struct Solver;

const NUM: u64 = 10_000;

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(NUM)
    }
}

fn solve(input: u64) -> u64 {
    // いちいち素因数分解してやるのは時間がかかりそうなので、エラトステネス風
    // に約数の和を計算してみる。
    let m = input as usize;
    let factors = (2..m).fold(repeat(1).take(m).collect::<Vec<usize>>(), |mut acc, i| {
        (i + i..m).step_by(i).for_each(|j| acc[j] += i);
        acc
    });

    (1..m)
        .filter_map(|i| {
            if i < 2 {
                return None;
            }

            let k = &factors[i];
            if k < &m && k != &i && factors[*k] == i {
                Some(i as u64)
            } else {
                None
            }
        })
        .sum()
}

#[allow(dead_code)]
fn solve2(input: u64) -> u64 {
    // 素直にやる
    (1..input).filter(|&n| has_amicable_number(n)).sum()
}

fn has_amicable_number(n: u64) -> bool {
    let d = sum_factors(n);
    if d < n || d - n == n {
        // d - n == n の場合は完全数なので false
        return false;
    }

    let e = sum_factors(d - n);
    e > d - n && e - (d - n) == n
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

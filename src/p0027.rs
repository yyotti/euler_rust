//! [Problem 27](https://projecteuler.net/problem=27)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2027))

use super::common::sieve;
use super::common::primes;

pub struct Solver;

const A: usize = 1_000;
const B: usize = 1_000;

impl super::Solver for Solver {
    fn solve(&self) -> i64 {
        solve(A, B)
    }
}

fn solve(a: usize, b: usize) -> i64 {
    // f(n) = n^2 + an + b とする。
    //
    // f(0) = b が素数にならなければならないので、bは素数である。
    //
    // また、f(n)が素数になるには f(n) > 0 でなければならない。
    // f(n)の判別式をDとすると
    //   D = a^2 - 4b < 0
    // つまり
    //   a^2 < 4b
    //   |a| < 2*sqrt(b)
    // である。

    sieve(b)
        .iter()
        .map(|&p| p as i64)
        .flat_map(|b| {
            let a_max = ((2.0 * (b as f64).sqrt()).floor() as i64).min(a as i64);
            (-(a_max - 1)..a_max).map(move |a| {
                let c = (0..)
                    .take_while(|&n| {
                        let t = n * n + a * n + b;
                        t.is_positive() && is_prime(t as u64)
                    })
                    .count();
                (a * b, c)
            })
        })
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(t, _)| t)
        .unwrap_or(0)
}

#[allow(dead_code)]
fn solve2(a: usize, b: usize) -> i64 {
    let p = a as i64;
    let q = b as i64;
    // 問題そのままやる
    (-(p - 1)..p)
        .flat_map(|a| {
            (-(q - 1)..q).map(move |b| {
                let c = (0..)
                    .take_while(|&n| {
                        let t = n * n + a * n + b;
                        t.is_positive() && is_prime(t as u64)
                    })
                    .count();
                (a * b, c)
            })
        })
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(t, _)| t)
        .unwrap_or(0)
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    primes()
        .take_while(|&p| p as f64 <= (n as f64).sqrt())
        .all(|p| n % p != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_price() {
        let ts = vec![
            (1, false),  //
            (2, true),   //
            (3, true),   //
            (4, false),  //
            (5, true),   //
            (6, false),  //
            (7, true),   //
            (8, false),  //
            (9, false),  //
            (10, false), //
            (11, true),  //
            (12, false), //
            (13, true),  //
            (14, false), //
            (15, false), //
            (16, false), //
            (17, true),  //
            (18, false), //
            (19, true),  //
            (20, false), //
        ];
        for (input, expected) in ts {
            assert_eq!(expected, is_prime(input), "n={}", input);
        }
    }

    #[test]
    fn test_solve() {
        assert_eq!(-41, solve(42, 42));
        assert_eq!(-1455, solve(100, 100));
        assert_eq!(-41, solve2(42, 42));
        assert_eq!(-1455, solve2(100, 100));
    }
}

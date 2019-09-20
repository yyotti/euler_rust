//! [Problem 27](https://projecteuler.net/problem=27)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2027))

use super::common::Primes;

pub struct Solver;

const A: isize = 1_000;
const B: isize = 1_000;

impl super::Solver for Solver {
    fn solve(&self) -> i64 {
        solve(A, B)
    }
}

fn solve(a: isize, b: isize) -> i64 {
    // 問題そのままやる
    (-a.abs()..=a)
        .flat_map(|a| {
            (-b..=b).map(move |b| {
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
        .unwrap_or(0) as i64
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    Primes::new()
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
        assert_eq!(-41, solve(41, 41));
        assert_eq!(-1455, solve(100, 100));
    }
}

//! [Problem 35](https://projecteuler.net/problem=35)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2035))

use super::common;

pub struct Solver;

const NUM: usize = 1_000_000;

impl super::Solver for Solver {
    fn solve(&self) -> i64 {
        solve(NUM)
    }
}

fn solve(max: usize) -> i64 {
    // まともにやる
    common::primes()
        .take_while(|&p| p < max as u64)
        .filter(|&p| {
            rotate_nums(p as usize)
                .iter()
                .all(|&n| common::is_prime(n as u64))
        })
        .count() as i64
}

fn rotate_nums(n: usize) -> Vec<usize> {
    let ds = common::digits(n as u64);
    (0..ds.len())
        .map(|i| {
            let mut ds = ds.clone();
            ds.rotate_left(i);
            common::digits_to_num(&ds.iter().map(|&d| d as usize).collect::<Vec<usize>>())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(13, solve(100));
    }

    #[test]
    fn test_rotate_nums() {
        let ts = vec![
            (1, vec![1]),               //
            (12, vec![12, 21]),         //
            (21, vec![21, 12]),         //
            (123, vec![123, 231, 312]), //
            (101, vec![101, 11, 110]),  //
        ];
        for (input, expected) in ts {
            assert_eq!(expected, rotate_nums(input));
        }
    }
}

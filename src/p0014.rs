//! [Problem 14](https://projecteuler.net/problem=14)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2014))

pub struct Solver;

const NUM: u64 = 1_000_000;

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(NUM)
    }
}

fn solve(input: u64) -> u64 {
    (1..input)
        .max_by_key(|&x| Collatz::new(x).count())
        .unwrap_or(0)
}

struct Collatz {
    n: Option<u64>,
}

impl Collatz {
    fn new(n: u64) -> Collatz {
        Collatz { n: Some(n) }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        match self.n {
            Some(1) => {
                self.n = None;
                Some(1)
            }
            Some(n) if n % 2 == 0 => {
                self.n = Some(n / 2);
                Some(n)
            },
            Some(n) => {
                self.n = Some(3 * n + 1);
                Some(n)
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz_new() {
        let c = Collatz::new(2);
        assert_eq!(Some(2), c.n);
    }

    #[test]
    fn test_collatz_next() {
        let expected = vec![13, 40, 20, 10, 5, 16, 8, 4, 2, 1];

        let mut c = Collatz::new(13);
        for ex in expected {
            assert_eq!(Some(ex), c.next());
        }
        assert_eq!(None, c.next());
    }

    #[test]
    fn test_solve() {
        let ts = vec![
            (1, 0),
            (2, 1),
            (5, 3),
            (10, 9),
        ];

        for (input, expected) in ts {
            assert_eq!(expected, solve(input));
        }
    }
}

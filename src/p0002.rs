//! [Problem 2](https://projecteuler.net/problem=2)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%202))

pub struct Solver;

const MAX_NUM: usize = 4_000_000;

impl super::Solver for Solver {
    fn solve(&self) -> i64 {
        solve(MAX_NUM)
    }
}

fn solve(input: usize) -> i64 {
    Fibonacci::new(1, 2)
        .take_while(|f| f <= &(input as u64))
        .filter(|f| f % 2 == 0)
        .sum::<u64>() as i64
}

struct Fibonacci {
    a1: u64,
    a2: u64,
}

impl Fibonacci {
    pub fn new(a1: u64, a2: u64) -> Fibonacci {
        Fibonacci { a1, a2 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let (a1, a2) = (self.a1, self.a2);

        self.a1 = a2;
        self.a2 = a1 + a2;

        Some(a1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_new() {
        let fib = Fibonacci::new(0, 1);
        assert_eq!((0, 1), (fib.a1, fib.a2));

        let fib = Fibonacci::new(1, 2);
        assert_eq!((1, 2), (fib.a1, fib.a2));
    }

    #[test]
    fn test_fibonacci_next() {
        let mut fib = Fibonacci { a1: 0, a2: 1 };
        for expected in vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34] {
            let f = fib.next();
            assert_eq!(Some(expected), f);
        }
    }

    #[test]
    fn test_solve() {
        let ts = vec![
            (10, 10),      //
            (100, 44),     //
            (1000, 798),   //
            (10000, 3382), //
        ];

        for (input, expected) in ts {
            assert_eq!(expected, solve(input));
        }
    }
}

//! [Problem 3](https://projecteuler.net/problem=3)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%203))

pub struct Solver;

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

struct Primes {
    ps: Vec<u64>,
    next: u64,
}

impl Primes {
    pub fn new() -> Primes {
        Primes {
            ps: vec![2],
            next: 2,
        }
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.next;

        match ((self.next + 1)..).find(|n| {
            let rt = (*n as f64).sqrt() as u64;
            self.ps
                .iter()
                .take_while(|m| *m <= &rt)
                .find(|m| *n % *m == 0)
                == None
        }) {
            Some(p) => {
                self.ps.push(p);
                self.next = p;
                Some(ret)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes_new() {
        let primes = Primes::new();
        assert_eq!(vec![2], primes.ps);
        assert_eq!(2, primes.next);
    }

    #[test]
    fn primes_next() {
        let mut primes = Primes {
            ps: vec![2],
            next: 2,
        };
        for expected in vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29] {
            let f = primes.next();
            assert_eq!(Some(expected), f);
        }
    }

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

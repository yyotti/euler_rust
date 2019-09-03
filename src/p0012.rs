//! [Problem 12](https://projecteuler.net/problem=12)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2012))

use super::common::prime_factors;

pub struct Solver;

const CNT: u32 = 500;

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve(CNT)
    }
}

fn solve(input: u32) -> u64 {
    Triangles::new()
        .find(|&t| count_factors(t) > input as u64)
        .unwrap_or(0)
}

fn count_factors(n: u64) -> u64 {
    prime_factors(n)
        .iter()
        .map(|(_, &e)| e as u64 + 1)
        .product()
}

struct Triangles {
    n: u64,
    t: u64,
}

impl Triangles {
    pub fn new() -> Triangles {
        Triangles { n: 1, t: 0 }
    }
}

impl Iterator for Triangles {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.t += self.n;
        self.n += 1;
        Some(self.t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangles_new() {
        let triangles = Triangles::new();
        assert_eq!(1, triangles.n);
        assert_eq!(0, triangles.t);
    }

    #[test]
    fn triangles_next() {
        let mut triangles = Triangles { n: 1, t: 0 };
        for expected in vec![1, 3, 6, 10, 15, 21, 28, 36, 45, 55] {
            let t = triangles.next();
            assert_eq!(Some(expected), t);
        }
    }

    #[test]
    fn get_factors_counts() {
        let ts = vec![
            (1, 1),
            (2, 2),
            (3, 2),
            (4, 3),
            (5, 2),
            (6, 4),
            (7, 2),
            (8, 4),
            (9, 3),
            (10, 4),
            (15, 4),
            (21, 4),
            (28, 6),
        ];
        for (input, expected) in ts {
            assert_eq!(expected, count_factors(input));
        }
    }

    #[test]
    fn find_highly_divisible_triangular_number() {
        let ts = vec![(1, 3), (2, 6), (3, 6), (4, 28), (5, 28)];
        for (input, expected) in ts {
            assert_eq!(expected, solve(input));
        }
    }
}

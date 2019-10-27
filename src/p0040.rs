//! [Problem 40](https://projecteuler.net/problem=40)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2040))

use super::common;

pub const INDICES: &[usize] = &[1, 10, 100, 1_000, 10_000, 100_000, 1_000_000];

pub fn solve(input: &[usize]) -> i64 {
    // イテレータを作って、指定されたインデックスの数字の積をとる。
    let mut c = Champernowne::new();
    input
        .iter()
        .zip(input.iter().skip(1))
        .fold(c.nth(input[0]).unwrap(), |acc, (&i1, &i2)| {
            acc * c.nth(i2 - i1 - 1).unwrap()
        }) as i64
}

struct Champernowne {
    n: usize,
    ds: Vec<usize>,
    idx: usize,
}

impl Champernowne {
    fn new() -> Champernowne {
        Champernowne {
            n: 0,
            ds: vec![0],
            idx: 0,
        }
    }
}

impl Iterator for Champernowne {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.ds.len() {
            self.n += 1;
            self.ds = common::digits(self.n as u64)
                .iter()
                .map(|&d| d as usize)
                .collect::<Vec<usize>>();
            self.idx = 0;
        }

        let res = Some(self.ds[self.idx]);
        self.idx += 1;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(768, solve(&[2, 4, 6, 8, 10, 12, 15]));
    }

    #[test]
    fn test_champernowne_new() {
        let actual = Champernowne::new();
        assert_eq!(0, actual.n);
        assert_eq!(vec![0], actual.ds);
        assert_eq!(0, actual.idx);
    }

    #[test]
    fn test_champernowne_next() {
        let digits = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 0, 1, 1, 1, 2, 1, 3, 1, 4, 1, 5,
        ];

        let mut c = Champernowne {
            n: 0,
            ds: vec![0],
            idx: 0,
        };

        for expected in digits {
            assert_eq!(Some(expected), c.next());
        }
    }
}

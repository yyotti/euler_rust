use std::collections::HashMap;

pub struct Primes {
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

pub fn prime_factors(n: u64) -> HashMap<u64, u32> {
    let mut map = HashMap::new();
    let mut m = n;
    for p in Primes::new() {
        if p > m {
            break;
        }

        while m % p == 0 {
            map.insert(p, map.get(&p).unwrap_or(&0) + 1);
            m /= p;
        }
    }
    map
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if a < b {
        return gcd(b, a);
    }

    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

pub fn multi(x: &Vec<u64>, y: u64) -> Vec<u64> {
    if x.is_empty() {
        return vec![0];
    }

    let (mut ds, mut c) = x.iter().rev().fold((vec![], 0), |(mut ds, c), d| {
        let z = d * y + c;
        ds.push(z % 10);
        (ds, z / 10)
    });
    while c > 0 {
        ds.push(c % 10);
        c /= 10;
    }
    ds.into_iter().rev().collect()
}

pub fn get_prime_factor_sums(max: u64) -> HashMap<u64, u64> {
    (1..=max).fold(HashMap::new(), |mut acc, i| {
        (2..=max / i).for_each(|j| {
            let n = i * j;
            match acc.get(&n) {
                Some(&k) => acc.insert(n, k + i),
                _ => acc.insert(n, i),
            };
        });
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes_new() {
        let primes = Primes::new();
        assert_eq!(vec![2], primes.ps);
        assert_eq!(2, primes.next);
    }

    #[test]
    fn test_primes_next() {
        let mut primes = Primes {
            ps: vec![2],
            next: 2,
        };
        for expected in vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29] {
            let p = primes.next();
            assert_eq!(Some(expected), p);
        }
    }

    #[test]
    fn test_prime_factors() {
        let ts = vec![
            (0, HashMap::new()),
            (1, HashMap::new()),
            (2, [(2, 1)].iter().cloned().collect()),
            (3, [(3, 1)].iter().cloned().collect()),
            (4, [(2, 2)].iter().cloned().collect()),
            (5, [(5, 1)].iter().cloned().collect()),
            (6, [(2, 1), (3, 1)].iter().cloned().collect()),
            (7, [(7, 1)].iter().cloned().collect()),
            (8, [(2, 3)].iter().cloned().collect()),
            (9, [(3, 2)].iter().cloned().collect()),
            (10, [(2, 1), (5, 1)].iter().cloned().collect()),
        ];
        for (input, expected) in ts {
            assert_eq!(expected, prime_factors(input));
        }
    }

    #[test]
    fn test_gcd() {
        let ts = vec![
            (1, (1, 0)),
            (1, (2, 1)),
            (2, (2, 2)),
            (1, (3, 1)),
            (1, (3, 2)),
            (3, (3, 3)),
            (1, (4, 1)),
            (2, (4, 2)),
            (1, (4, 3)),
            (4, (4, 4)),
            (1, (6, 1)),
            (2, (6, 2)),
            (3, (6, 3)),
            (2, (6, 4)),
            (1, (6, 5)),
            (6, (6, 6)),
            (6, (6, 6)),
        ];
        for (expected, (a, b)) in ts {
            assert_eq!(expected, gcd(a, b), "gcd({}, {})", a, b);
            assert_eq!(expected, gcd(b, a), "gcd({}, {})", b, a);
        }
    }

    #[test]
    fn test_multi() {
        let ts = vec![
            (vec![], 10, vec![0]),
            (vec![2], 1, vec![2]),
            (vec![3], 3, vec![9]),
            (vec![3], 4, vec![1, 2]),
            (vec![2, 0], 4, vec![8, 0]),
            (vec![2, 0], 5, vec![1, 0, 0]),
            (vec![1, 2, 3], 17, vec![2, 0, 9, 1]),
            (vec![9, 9, 9], 999, vec![9, 9, 8, 0, 0, 1]),
        ];

        for (in_x, in_y, expected) in ts {
            assert_eq!(
                expected,
                multi(&in_x, in_y),
                "{} x {}",
                &in_x.iter().fold(0, |acc, d| acc * 10 + d),
                in_y
            );
        }
    }

    #[test]
    fn test_get_prime_factor_sums() {
        let expected = vec![
            (2, 1),
            (3, 1),
            (4, 3),
            (5, 1),
            (6, 6),
            (7, 1),
            (8, 7),
            (9, 4),
            (10, 8),
            (11, 1),
            (12, 16),
            (13, 1),
            (14, 10),
            (15, 9),
            (16, 15),
            (17, 1),
            (18, 21),
            (19, 1),
            (20, 22),
        ];
        assert_eq!(
            expected.iter().cloned().collect::<HashMap<u64, u64>>(),
            get_prime_factor_sums(20)
        );
    }
}

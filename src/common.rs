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
            let p = primes.next();
            assert_eq!(Some(expected), p);
        }
    }

    #[test]
    fn get_prime_factors() {
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
}

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
}

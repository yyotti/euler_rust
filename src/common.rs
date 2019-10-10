use std::collections::HashMap;
use std::ops::Rem;

pub fn primes() -> Box<dyn Iterator<Item = u64>> {
    let mut multiples = HashMap::new();
    let iter = (3..).step_by(2).filter_map(move |i| {
        let (prime_or_none, factor) = match multiples.remove(&i) {
            Some(f) => (None, f),
            None => (Some(i), i * 2),
        };

        (1..)
            .map(|j| i + j * factor)
            .skip_while(|m| multiples.contains_key(m))
            .next()
            .map(|m| multiples.insert(m, factor));

        prime_or_none
    });

    Box::new((2..).take(1).chain(iter))
}

pub fn sieve(m: usize) -> Vec<u64> {
    (0..=m)
        .fold((0..=m).collect::<Vec<usize>>(), |mut acc, i| {
            if i < 2 {
                return acc;
            }
            (i + i..acc.len()).step_by(i).for_each(|i| acc[i] /= i);
            acc
        })
        .iter()
        .filter(|&&p| p >= 2)
        .map(|&p| p as u64)
        .collect()
}

pub fn prime_factors(n: u64) -> HashMap<u64, usize> {
    primes()
        .scan(n, |m, p| {
            if p > *m {
                return None;
            }

            let mut c = 0;
            while *m % p == 0 {
                c += 1;
                *m /= p;
            }

            Some((p, c))
        })
        .filter(|&(_, c)| c > 0)
        .collect()
}

pub fn gcd<T>(a: T, b: T) -> T
where
    T: PartialOrd + Default + Rem<Output = T> + Copy,
{
    if a < b {
        return gcd(b, a);
    }

    if b == T::default() {
        return a;
    }

    gcd(b, a % b)
}

// TODO ジェネリックにできないか？もしくはVec<usize>とかでいいのでは？
// TODO scanで何とかできそうな気はする
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

// TODO ジェネリックにできないか？
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

pub fn sum_string_int(a: &str, b: &str) -> String {
    let m = a.len().max(b.len());

    let num1 = padding(a, '0', m, true);
    let num2 = padding(b, '0', m, true);

    let (mut s, c) = (0..m)
        .rev()
        .map(|i| {
            (
                num1[i..i + 1].parse::<u8>().unwrap(),
                num2[i..i + 1].parse::<u8>().unwrap(),
            )
        })
        .fold((String::from(""), 0), |(mut s, c), (n1, n2)| {
            let k = n1 + n2 + c;
            s.insert_str(0, &(k % 10).to_string());
            (s, k / 10)
        });

    if c > 0 {
        s.insert_str(0, &c.to_string());
    }

    s
}

fn padding(s: &str, c: char, total_len: usize, left: bool) -> String {
    if s.len() >= total_len {
        return String::from(s);
    }

    let mut new_s = String::from(s);
    let pad = c.to_string().repeat(total_len - s.len());
    if left {
        new_s.insert_str(0, &pad);
    } else {
        new_s.push_str(&pad);
    }
    new_s
}

pub fn digits(n: u64) -> Vec<u64> {
    if n < 10 {
        return vec![n];
    }

    let mut v = digits(n / 10);
    v.push(n % 10);
    v
}

pub fn fact(n: usize) -> u64 {
    if n == 0 {
        return 1;
    }

    n as u64 * fact(n - 1)
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    primes()
        .take_while(|&p| p as f64 <= (n as f64).sqrt())
        .all(|p| n % p != 0)
}

pub fn digits_to_num(ds: &[usize]) -> usize {
    ds.iter().fold(0, |acc, d| acc * 10 + d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes() {
        let mut primes = primes();
        for expected in vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29] {
            let p = primes.next();
            assert_eq!(Some(expected), p);
        }
    }

    #[test]
    fn test_sieve() {
        assert_eq!(vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29], sieve(30));
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

    #[test]
    fn test_sum_string_int() {
        let ts = vec![
            (
                "20849603980134001723930671666823555245252804609722",
                "53503534226472524250874054075591789781264330331690",
                "74353138206606525974804725742415345026517134941412",
            ),
            (
                "77158542502016545090413245809786882778948721859617",
                "72107838435069186155435662884062257473692284509516",
                "149266380937085731245848908693849140252641006369133",
            ),
            ("0", "0", "0"),
            ("1", "0", "1"),
            ("1", "1", "2"),
            ("5", "4", "9"),
            ("5", "5", "10"),
            ("14", "5", "19"),
            ("345", "655", "1000"),
        ];
        for (a, b, expected) in ts {
            assert_eq!(expected, sum_string_int(a, b));
        }
    }

    #[test]
    fn test_padding() {
        let ts = vec![
            ("", ' ', 0, true, ""),
            ("", ' ', 0, false, ""),
            ("", ' ', 1, true, " "),
            ("", ' ', 2, false, "  "),
            ("a", 'b', 3, true, "bba"),
            ("a", 'b', 2, false, "ab"),
            ("abc", 'd', 3, true, "abc"),
            ("abc", 'd', 2, false, "abc"),
        ];
        for (s, c, n, left, expected) in ts {
            assert_eq!(expected, padding(s, c, n, left));
        }
    }

    #[test]
    fn test_digits() {
        let ts = vec![
            (0, vec![0]),             //
            (1, vec![1]),             //
            (2, vec![2]),             //
            (3, vec![3]),             //
            (4, vec![4]),             //
            (5, vec![5]),             //
            (6, vec![6]),             //
            (7, vec![7]),             //
            (8, vec![8]),             //
            (9, vec![9]),             //
            (10, vec![1, 0]),         //
            (11, vec![1, 1]),         //
            (12, vec![1, 2]),         //
            (21, vec![2, 1]),         //
            (22, vec![2, 2]),         //
            (100, vec![1, 0, 0]),     //
            (101, vec![1, 0, 1]),     //
            (120, vec![1, 2, 0]),     //
            (121, vec![1, 2, 1]),     //
            (1001, vec![1, 0, 0, 1]), //
            (1221, vec![1, 2, 2, 1]), //
        ];
        for (input, expected) in ts {
            assert_eq!(expected, digits(input))
        }
    }

    #[test]
    fn test_fact() {
        let ts = vec![
            (0, 1),   // 0!
            (1, 1),   // 1!
            (2, 2),   // 2!
            (3, 6),   // 3!
            (4, 24),  // 4!
            (5, 120), // 5!
        ];
        for (input, expected) in ts {
            assert_eq!(expected, fact(input));
        }
    }

    #[test]
    fn test_is_prime() {
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
    fn test_digits_to_num() {
        let ts = vec![
            (vec![], 0),
            (vec![0], 0),
            (vec![1], 1),
            (vec![0, 2], 2),
            (vec![1, 0], 10),
            (vec![1, 0, 2], 102),
            (vec![3, 2, 1], 321),
        ];
        for (input, expected) in ts {
            assert_eq!(expected, digits_to_num(&input));
        }
    }
}

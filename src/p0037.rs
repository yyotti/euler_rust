//! [Problem 37](https://projecteuler.net/problem=37)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2037))

use super::common;
use std::collections::HashMap;

// TODO 固定値を与えられているのはなんか気持ち悪い
const MAX_CNT: usize = 11;

pub fn solve() -> i64 {
    // 真面目に1つずつ素数を調べていく。
    //
    // 例えば251は素数だが、左の1桁を落とした51は素数ではない。
    // そして2251も素数だが、左の1桁を落とした251は切り捨て素数ではないと判明
    // しているので、その時点でチェックを打ち切ることができる。
    // 逆に、もし251が切り捨て素数であるなら、2251も左は切り捨て素数であると
    // 判定できる。

    let mut cnt = 0;
    let mut sum = 0;

    let mut cache = HashMap::new();
    let mut ps = common::primes().skip_while(|&p| p < 10);

    while cnt < MAX_CNT {
        let p = ps.next().unwrap();
        if is_dropable_prime(p, &mut cache) {
            sum += p;
            cnt += 1;
        }
        if cnt >= MAX_CNT {
            break;
        }
    }

    sum as i64
}

fn is_dropable_prime(p: u64, cache: &mut HashMap<u64, bool>) -> bool {
    drop_numbers(p as usize).iter().all(|&n| {
        if cache.contains_key(&(n as u64)) {
            return *cache.get(&(n as u64)).unwrap();
        }

        let res = common::is_prime(n as u64);
        cache.insert(n as u64, res);
        res
    })
}

fn drop_numbers(n: usize) -> Vec<usize> {
    let ds = common::digits(n as u64)
        .iter()
        .map(|&d| d as usize)
        .collect::<Vec<usize>>();

    let mut ns = vec![n];
    ns.append(&mut drop_lefts(ds.as_slice()));
    ns.append(&mut drop_rights(ds.as_slice()));

    ns
}

fn drop_lefts(ds: &[usize]) -> Vec<usize> {
    if ds.len() == 0 {
        return vec![];
    }

    let mut ns = vec![];
    for i in 1..ds.len() {
        ns.push(common::digits_to_num(&ds[i..]))
    }

    ns
}

fn drop_rights(ds: &[usize]) -> Vec<usize> {
    if ds.len() == 0 {
        return vec![];
    }

    let mut ns = vec![];
    for i in (1..ds.len()).rev() {
        ns.push(common::digits_to_num(&ds[..i]))
    }

    ns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        // No tests
    }

    #[test]
    fn test_drop_lefts() {
        let ts = vec![
            (vec![], vec![]),
            (vec![1], vec![]),
            (vec![1, 2], vec![2]),
            (vec![1, 2, 3], vec![23, 3]),
            (vec![1, 0, 2, 3], vec![23, 23, 3]),
            (vec![1, 0, 2, 3, 0], vec![230, 230, 30, 0]),
        ];
        for (input, expected) in ts {
            assert_eq!(expected, drop_lefts(&input));
        }
    }

    #[test]
    fn test_drop_rights() {
        let ts = vec![
            (vec![], vec![]),
            (vec![1], vec![]),
            (vec![1, 2], vec![1]),
            (vec![1, 2, 3], vec![12, 1]),
            (vec![1, 0, 2, 3], vec![102, 10, 1]),
            (vec![1, 0, 2, 3, 0], vec![1023, 102, 10, 1]),
        ];
        for (input, expected) in ts {
            assert_eq!(expected, drop_rights(&input));
        }
    }

    #[test]
    fn test_drop_numbers() {
        let ts = vec![
            (0, vec![0]),
            (1, vec![1]),
            (12, vec![12, 2, 1]),
            (123, vec![123, 23, 3, 12, 1]),
            (1023, vec![1023, 23, 23, 3, 102, 10, 1]),
            (10230, vec![10230, 230, 230, 30, 0, 1023, 102, 10, 1]),
        ];
        for (input, expected) in ts {
            assert_eq!(expected, drop_numbers(input));
        }
    }

    #[test]
    fn test_is_dropable_prime() {
        let ts = vec![
            (11, false),   //
            (37, true),    //
            (3797, true),  //
            (3347, false), //
        ];
        for (input, expected) in ts {
            assert_eq!(expected, is_dropable_prime(input, &mut HashMap::new()));
        }
    }
}

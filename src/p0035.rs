//! [Problem 35](https://projecteuler.net/problem=35)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2035))

use super::common;
use std::collections::HashSet;

pub const MAX_NUM: usize = 1_000_000;

pub fn solve(max: usize) -> i64 {
    // 最大値が分かっているので、あらかじめそこまでの素数を求めておいて素数判定
    // を行う方が早い。
    // また、例えば 197 が巡回素数であればそれを巡回させた 971,719 も巡回素数
    // であるから、キャッシュしておいてチェックに利用できる。
    //
    // 巡回させて全て素数にならなければならないので、各桁が全て奇数でなければ
    // ならないという条件もあるが、それはやってもやらなくても速度にあまり影響は
    // ない。

    // あえて手続き型風で書いてみる
    let primes: HashSet<u64> = common::sieve(max).into_iter().collect();
    let mut cache: HashSet<u64> = HashSet::new();
    let mut cnt = 0;

    for &p in &primes {
        if cache.contains(&p) {
            cnt += 1;
            continue;
        }

        let rotated_nums = rotate_nums(p as usize);
        if rotated_nums.iter().all(|&n| primes.contains(&(n as u64))) {
            cache.extend(rotated_nums.iter().map(|&p| p as u64));
            cnt += 1;
        }
    }

    cnt as i64
}

#[allow(dead_code)]
fn solve2(max: usize) -> i64 {
    // まともにやる
    common::primes()
        .take_while(|&p| p < max as u64)
        .filter(|&p| {
            rotate_nums(p as usize)
                .iter()
                .all(|&n| common::is_prime(n as u64))
        })
        .count() as i64
}

fn rotate_nums(n: usize) -> HashSet<usize> {
    let ds = common::digits(n as u64);
    (0..ds.len())
        .map(|i| {
            let mut ds = ds.clone();
            ds.rotate_left(i);
            common::digits_to_num(&ds.iter().map(|&d| d as usize).collect::<Vec<usize>>())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(13, solve(100));
        assert_eq!(13, solve2(100));
    }

    #[test]
    fn test_rotate_nums() {
        let ts = vec![
            (1, vec![1].into_iter().collect::<HashSet<usize>>()), //
            (11, vec![11].into_iter().collect()),                 //
            (12, vec![12, 21].into_iter().collect()),             //
            (21, vec![21, 12].into_iter().collect()),             //
            (123, vec![123, 231, 312].into_iter().collect()),     //
            (101, vec![101, 11, 110].into_iter().collect()),      //
        ];
        for (input, expected) in ts {
            assert_eq!(expected, rotate_nums(input));
        }
    }
}

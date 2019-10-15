//! [Problem 32](https://projecteuler.net/problem=32)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2032))

use std::collections::HashSet;
use super::common::digits_to_num;

pub const N: usize = 9;

pub fn solve(input: usize) -> i64 {
    if input < 3 {
        return 0;
    }

    // 1～inputの各数字を並べた数列で順列をとり、3パートに分けて数値とみなして
    // 条件を満たすか判定する。
    //
    //   a * b = c
    // とすると、
    //   ・aとbを入れ替えても結果は同じ
    //   ・aとbの積の桁数は、aの桁数とbの桁数のうち大きい方と同じかそれ以上となる
    // ということから、パートの分け方は
    //   aの桁数 <= bの桁数 <= cの桁数
    // となる場合のみを調べればよい。

    let positions: Vec<(usize, usize)> = combinations(&(1..input).collect::<Vec<usize>>(), 2)
        .iter()
        .filter_map(|indices| {
            let a_len = indices[0];
            let b_len = indices[1] - a_len;
            let c_len = input - (a_len + b_len);
            if a_len <= b_len && b_len <= c_len {
                Some((indices[0], indices[1]))
            } else {
                None
            }
        })
        .collect();

    permutations(&(1..=input).collect::<Vec<usize>>(), input)
        .iter()
        .fold(HashSet::new(), |mut acc, nums| {
            positions.iter().for_each(|&(i1, i2)| {
                let a = digits_to_num(&nums[..i1]);
                let b = digits_to_num(&nums[i1..i2]);
                let c = digits_to_num(&nums[i2..]);
                if a * b == c {
                    acc.insert(c);
                }
            });

            acc
        })
        .iter()
        .sum::<usize>() as i64
}

fn permutations<T>(elems: &[T], r: usize) -> Vec<Vec<T>>
where
    T: Copy,
{
    if r == 0 {
        return vec![vec![]];
    }

    elems
        .split_first()
        .map(|(&e, tail)| {
            permutations(&tail, r - 1)
                .iter()
                .flat_map(|es| {
                    (0..=es.len()).map(move |i| {
                        let mut v = es.clone();
                        v.insert(i, e);
                        v
                    })
                })
                .chain(permutations(&tail, r).into_iter())
                .collect()
        })
        .unwrap_or(vec![])
}

fn combinations<T>(elems: &[T], r: usize) -> Vec<Vec<T>>
where
    T: Copy,
{
    if r == 0 {
        return vec![vec![]];
    }

    elems
        .split_first()
        .map(|(&e, tail)| {
            combinations(&tail, r - 1)
                .iter_mut()
                .map(|es| {
                    es.insert(0, e);
                    es.clone()
                })
                .chain(combinations(&tail, r).into_iter())
                .collect()
        })
        .unwrap_or(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let ts = vec![
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 12),  // 3 * 4 = 12
            (5, 52),  // 4 * 13 = 52
            (6, 162), // 3 * 54 = 162
            (7, 0),
        ];
        for (input, expected) in ts {
            assert_eq!(expected, solve(input));
        }
    }

    #[test]
    fn test_permutations() {
        let ts = vec![
            (vec![], 0, vec![vec![]]),
            (vec![], 1, vec![]),
            (vec![1], 0, vec![vec![]]),
            (vec![2], 1, vec![vec![2]]),
            (vec![2], 2, vec![]),
            (vec![1, 2], 0, vec![vec![]]),
            (vec![1, 2], 1, vec![vec![1], vec![2]]),
            (vec![1, 2], 2, vec![vec![1, 2], vec![2, 1]]),
            (vec![1, 2], 3, vec![]),
            (vec![1, 2, 3], 0, vec![vec![]]),
            (vec![1, 2, 3], 1, vec![vec![1], vec![2], vec![3]]),
            (
                vec![1, 2, 3],
                2,
                vec![
                    vec![1, 2],
                    vec![2, 1],
                    vec![1, 3],
                    vec![3, 1],
                    vec![2, 3],
                    vec![3, 2],
                ],
            ),
            (
                vec![1, 2, 3],
                3,
                vec![
                    vec![1, 2, 3],
                    vec![2, 1, 3],
                    vec![2, 3, 1],
                    vec![1, 3, 2],
                    vec![3, 1, 2],
                    vec![3, 2, 1],
                ],
            ),
            (vec![1, 2, 3], 4, vec![]),
        ];
        for (elems, r, expected) in ts {
            assert_eq!(
                expected,
                permutations(&elems, r),
                "elems={:?}, r={}",
                elems,
                r
            );
        }
    }

    #[test]
    fn test_combinations() {
        let ts = vec![
            (vec![], 0, vec![vec![]]),
            (vec![], 1, vec![]),
            (vec![1], 0, vec![vec![]]),
            (vec![2], 1, vec![vec![2]]),
            (vec![2], 2, vec![]),
            (vec![1, 2], 0, vec![vec![]]),
            (vec![1, 2], 1, vec![vec![1], vec![2]]),
            (vec![1, 2], 2, vec![vec![1, 2]]),
            (vec![1, 2], 3, vec![]),
            (vec![1, 2, 3], 0, vec![vec![]]),
            (vec![1, 2, 3], 1, vec![vec![1], vec![2], vec![3]]),
            (vec![1, 2, 3], 2, vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
            (vec![1, 2, 3], 3, vec![vec![1, 2, 3]]),
            (vec![1, 2, 3], 4, vec![]),
            (vec![1, 2, 3, 4], 0, vec![vec![]]),
            (
                vec![1, 2, 3, 4],
                1,
                vec![vec![1], vec![2], vec![3], vec![4]],
            ),
            (
                vec![1, 2, 3, 4],
                2,
                vec![
                    vec![1, 2],
                    vec![1, 3],
                    vec![1, 4],
                    vec![2, 3],
                    vec![2, 4],
                    vec![3, 4],
                ],
            ),
            (
                vec![1, 2, 3, 4],
                3,
                vec![vec![1, 2, 3], vec![1, 2, 4], vec![1, 3, 4], vec![2, 3, 4]],
            ),
            (vec![1, 2, 3, 4], 4, vec![vec![1, 2, 3, 4]]),
            (vec![1, 2, 3, 4], 5, vec![]),
        ];
        for (elems, r, expected) in ts {
            assert_eq!(
                expected,
                combinations(&elems, r),
                "elems={:?}, r={}",
                elems,
                r
            );
        }
    }
}

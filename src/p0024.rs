//! [Problem 24](https://projecteuler.net/problem=24)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2024))

use super::common::fact;

pub const NUMS: &[u64] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
pub const N: usize = 1_000_000;

pub fn solve(input: &[u64], n: usize) -> i64 {
    // input = [a1, a2, ... , ak] とし、 a1 < a2 < ... < ak を満たすものとする。
    //
    // a1を先頭に置いた場合、残りの(k-1)個の要素の並べ方は (k-1)! 通りある。
    // この時、
    //   (k-1)! < n であれば、a2以降を先頭に置いた場合に解がある
    //   (k-1)! >= n であれば、a1を先頭に置いた場合に解がある
    // まず先頭の要素がどれになるかを確定したら、同様の手順でa2以降の要素を
    // 再帰的に確定していけばよい。

    let mut perm: Vec<u64> = vec![];
    let mut elems = input.to_vec();
    let mut k = n as u64 - 1;
    while elems.len() > 0 {
        let f = fact(elems.len() - 1);
        perm.push(elems.remove((k / f) as usize));
        k %= f;
    }

    perm.iter().fold(0, |acc, d| acc * 10 + d) as i64
}

#[allow(dead_code)]
fn solve2(input: &[u64], n: usize) -> u64 {
    // 普通に順列を作ってやる
    DicPerm::new(input)
        .skip(n - 1)
        .next()
        .unwrap_or(vec![])
        .iter()
        .fold(0, |acc, d| acc * 10 + d)
}

struct DicPerm<T>
where
    T: Ord + Clone,
{
    elems: Vec<T>,
    res: Option<Vec<T>>,
}

impl<T> DicPerm<T>
where
    T: Ord + Clone,
{
    fn new(elems: &[T]) -> DicPerm<T> {
        let mut vec = elems.to_vec();
        vec.sort();
        DicPerm {
            elems: vec.clone(),
            res: Some(vec),
        }
    }
}

impl<T> Iterator for DicPerm<T>
where
    T: Ord + Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.res.is_none() {
            return None;
        }

        let mut elems = self.elems.clone();

        match elems
            .clone()
            .iter()
            .zip(elems.iter().skip(1))
            .rposition(|(e1, e2)| e1 < e2)
        {
            Some(idx1) => {
                let idx2 = elems.iter().rposition(|e| e > &elems[idx1]).unwrap();

                elems.swap(idx1, idx2);
                elems.splice(
                    idx1 + 1..,
                    elems.clone().iter().skip(idx1 + 1).rev().cloned(),
                );

                self.res = Some(self.elems.clone());
                self.elems = elems;

                self.res.clone()
            }
            _ => {
                let res = Some(elems.clone());
                self.res = None;
                res
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dic_perm_new() {
        let p = DicPerm::new(&[3, 2, 4, 1]);
        assert_eq!(vec![1, 2, 3, 4], p.elems);
        assert_eq!(Some(vec![1, 2, 3, 4]), p.res);
    }

    #[test]
    fn test_dic_perm_next() {
        let mut p = DicPerm {
            elems: vec![1, 2, 3, 4],
            res: Some(vec![1, 2, 3, 4]),
        };
        let ts = vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 4, 3],
            vec![1, 3, 2, 4],
            vec![1, 3, 4, 2],
            vec![1, 4, 2, 3],
            vec![1, 4, 3, 2],
            vec![2, 1, 3, 4],
            vec![2, 1, 4, 3],
            vec![2, 3, 1, 4],
            vec![2, 3, 4, 1],
            vec![2, 4, 1, 3],
            vec![2, 4, 3, 1],
            vec![3, 1, 2, 4],
            vec![3, 1, 4, 2],
            vec![3, 2, 1, 4],
            vec![3, 2, 4, 1],
            vec![3, 4, 1, 2],
            vec![3, 4, 2, 1],
            vec![4, 1, 2, 3],
            vec![4, 1, 3, 2],
            vec![4, 2, 1, 3],
            vec![4, 2, 3, 1],
            vec![4, 3, 1, 2],
            vec![4, 3, 2, 1],
        ];

        for es in ts {
            assert_eq!(Some(es), p.next());
        }
        assert_eq!(None, p.next());
    }

    #[test]
    fn test_solve() {
        assert_eq!(3021, solve(&[0, 1, 2, 3], 20));
        assert_eq!(3021, solve2(&[0, 1, 2, 3], 20));
    }
}

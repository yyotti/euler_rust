//! [Problem 28](https://projecteuler.net/problem=28)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2028))

use std::iter::repeat;

pub struct Solver;

const SIZE: usize = 1001;

impl super::Solver for Solver {
    fn solve(&self) -> i64 {
        solve(SIZE)
    }
}

fn solve(input: usize) -> i64 {
    if input % 2 == 0 {
        return 0;
    }

    // 問題通りに正方形を作って対角線の和をとる
    let mut square: Vec<Vec<u64>> = repeat(repeat(0).take(input).collect())
        .take(input)
        .collect();

    let center = input / 2;
    square[center][center] = 1;

    let distances = vec![
        (0, 1),  // right
        (1, 0),  // down
        (0, -1), // left
        (-1, 0), // up
    ];

    let square = (2..=input * input)
        .fold(((center, center), 0, &mut square), |((x, y), i, s), n| {
            let d = distances[i];
            let mut new_x = x as isize + d.0;
            let mut new_y = y as isize + d.1;
            if new_x < 0 || new_x >= input as isize || new_y < 0 || new_y >= input as isize {
                let d = distances[(i + 1) % distances.len()];
                new_x = x as isize + d.0;
                new_y = y as isize + d.1;
            }
            s[new_x as usize][new_y as usize] = n as u64;

            let check_i = (i + 1) % distances.len();
            let check_d = distances[check_i];
            let check_x = new_x as isize + check_d.0;
            let check_y = new_y as isize + check_d.1;

            let next_i = if check_x < 0
                || check_x >= input as isize
                || check_y < 0
                || check_y >= input as isize
                || s[check_x as usize][check_y as usize] == 0
            {
                check_i
            } else {
                i
            };

            ((new_x as usize, new_y as usize), next_i, s)
        })
        .2;

    (0..input)
        .map(|i| (square[i][i] + square[i][input - i - 1]) as i64)
        .sum::<i64>()
        - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let ts = vec![
            (1, 1),   //
            (2, 0),   //
            (3, 25),  //
            (4, 0),   //
            (5, 101), //
            (6, 0),   //
            (7, 261), //
        ];
        for (input, expected) in ts {
            assert_eq!(expected, solve(input));
        }
    }
}

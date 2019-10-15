//! [Problem 28](https://projecteuler.net/problem=28)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2028))

use std::iter::repeat;

pub const SIZE: usize = 1001;

pub fn solve(input: usize) -> i64 {
    // 作成する正方形の1辺の大きさをN(Nは正の奇数)とする。
    // N = 1の場合は1のみで他に正方形の角にあたる点は無い。
    // N >= 3の場合に正方形の対角線上に位置する数字がどうなるかを考える。
    //
    // 1辺がNの正方形なので、配置されている数字の最大値はN^2であり、その位置は
    // 右上の角である。
    // そこから反時計回りに(N-1)だけ戻った位置が左上の角で、配置されている数字
    // は N^2 - (N-1) である。
    // 同様に左下と右下の角に配置されている数字を計算でき、全ての角に配置されて
    // いる数字を列挙すると
    //   N^2             = N^2 - 0 * (N-1)
    //   N^2 - N-1       = N^2 - 1 * (N-1)
    //   N^2 - 2 * (N-1) = N^2 - 2 * (N-1)
    //   N^2 - 3 * (N-1) = N^2 - 3 * (N-1)
    // となる。和をとれば
    //   N^2 * 4 - (N-1) * (0+1+2+3) = 4N^2 - 6(N-1)
    //                               = 4N^2 - 6N + 6
    // となり、これが最も外側の角に配置されている数字の和である。
    //
    // 正方形の最も外側の数字を全て取り除くと、サイズが(N-2)の正方形になる。
    // N=N-2 として考えれば上記と同じように角の数字の和が計算できる。
    // サイズNの正方形の対角線上には、中心の1を除けばサイズ3～Nの正方形の角の
    // 数字が並んでいる。つまり
    //   S(N) = 4N^2 - 6N + 6
    // とすれば、対角線上の数字の和T(N)は
    //   T(N) = Σ(k=3～N) {S(N)} + 1
    //        = Σ(k=3～N) {4N^2 - 6N + 6} + 1
    // となる。
    // Nは3以上の奇数であるから、nを自然数として
    //   N = 2n + 1
    // と表せるので、
    //  T(n) = Σ(n=1～(N-1)/2) {4(2n+1)^2 - 6(2n+1) + 6} + 1
    //       = Σ(n=1～(N-1)/2) {4(4n^2 + 4n + 1) - 6(2n+1) + 6} + 1
    //       = Σ(n=1～(N-1)/2) {16n^2 + 16n + 4 - (12n + 6) + 6} + 1
    //       = Σ(n=1～(N-1)/2) {16n^2 + 4n + 4} + 1
    //       = Σ(n=1～(N-1)/2) {4(4n^2 + n + 1)} + 1
    //       = 4 * Σ(n=1～(N-1)/2) {4n^2 + n + 1} + 1
    //       (t = (N-1)/2 として)
    //       = 4 * {4(t(t+1)(2t+1)/6) + (t(t+1)/2) + t} + 1
    //       = 4 * {4((2t^3 + 3t^2 + t)/6) + (t^2 + t)/2 + t} + 1
    //       = 4 * {(8t^3 + 12t^2 + 4t + 3t^2 + 3t + 6t)/6} + 1
    //       = 2(8t^3 + 15t^2 + 13t)/3 + 1
    //       = (16t^3 + 30t^2 + 26t + 3)/3
    if input % 2 == 0 {
        return 0;
    }

    let t = (input as u64 - 1) / 2;
    ((16 * t * t * t + 30 * t * t + 26 * t + 3) / 3) as i64
}

#[allow(dead_code)]
fn solve2(input: usize) -> i64 {
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
            assert_eq!(expected, solve2(input));
        }
    }
}

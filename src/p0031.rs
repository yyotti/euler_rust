//! [Problem 31](https://projecteuler.net/problem=31)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2031))

pub struct Solver;

const N: usize = 200;

const COINS: &[usize] = &[200, 100, 50, 20, 10, 5, 2, 1];

impl super::Solver for Solver {
    fn solve(&self) -> i64 {
        solve(N, COINS)
    }
}

fn solve(input: usize, coins: &[usize]) -> i64 {
    if input == 0 {
        return 1;
    }

    let mut cs = coins.iter().skip_while(|&&c| c > input);

    cs.next()
        .map(|coin| {
            let tail = cs.copied().collect::<Vec<usize>>();
            (1..=input / coin)
                .map(|i| solve(input - i * coin, &tail))
                .sum::<i64>()
                + solve(input, &tail)
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let coins = &[2, 1];
        assert_eq!(1, solve(0, coins), "coins={:?}, input={}", coins, 0);
        assert_eq!(1, solve(1, coins), "coins={:?}, input={}", coins, 1);
        assert_eq!(2, solve(2, coins), "coins={:?}, input={}", coins, 2);
        assert_eq!(2, solve(3, coins), "coins={:?}, input={}", coins, 3);
        assert_eq!(3, solve(4, coins), "coins={:?}, input={}", coins, 4);

        assert_eq!(11, solve(10, COINS), "coins={:?}, input={}", COINS, 10);

        let coins = &[2];
        assert_eq!(1, solve(0, coins), "coins={:?}, input={}", coins, 0);
        assert_eq!(0, solve(1, coins), "coins={:?}, input={}", coins, 1);
        assert_eq!(1, solve(2, coins), "coins={:?}, input={}", coins, 2);
        assert_eq!(0, solve(3, coins), "coins={:?}, input={}", coins, 3);
    }
}

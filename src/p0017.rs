//! [Problem 17](https://projecteuler.net/problem=17)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2017))

use std::collections::HashMap;

pub struct Solver;

const NUM: usize = 1000;

const NUM_WORD_LEN: &[(usize, usize)] = &[
    (1, "one".len()),
    (2, "two".len()),
    (3, "three".len()),
    (4, "four".len()),
    (5, "five".len()),
    (6, "six".len()),
    (7, "seven".len()),
    (8, "eight".len()),
    (9, "nine".len()),
    (10, "ten".len()),
    (11, "eleven".len()),
    (12, "twelve".len()),
    (13, "thirteen".len()),
    (14, "fourteen".len()),
    (15, "fifteen".len()),
    (16, "sixteen".len()),
    (17, "seventeen".len()),
    (18, "eighteen".len()),
    (19, "nineteen".len()),
    (20, "twelve".len()),
    (30, "thirty".len()),
    (40, "forty".len()),
    (50, "fifty".len()),
    (60, "sixty".len()),
    (70, "seventy".len()),
    (80, "eighty".len()),
    (90, "ninety".len()),
    (1000, "onethousand".len()),
];

const HUNDRED_LEN: usize = "hundred".len();

const AND_LEN: usize = "and".len();

impl super::Solver for Solver {
    fn solve(&self) -> i64 {
        solve(NUM)
    }
}

fn solve(input: usize) -> i64 {
    let word_lens: HashMap<usize, usize> = NUM_WORD_LEN.iter().copied().collect();
    (1..=input).fold(0, |acc, n| acc + num_word_len(n, &word_lens)) as i64
}

fn num_word_len(n: usize, word_lens: &HashMap<usize, usize>) -> usize {
    match n {
        n if word_lens.contains_key(&n) => *word_lens.get(&n).unwrap(),
        n if n >= 100 => {
            let k = word_lens.get(&(n / 100)).unwrap_or(&0) + HUNDRED_LEN;
            if n % 100 == 0 {
                return k;
            }
            k + AND_LEN + num_word_len(n % 100, word_lens)
        }
        _ => {
            let k = n / 10;
            word_lens.get(&(k * 10)).unwrap_or(&0) + word_lens.get(&(n % 10)).unwrap_or(&0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(19, solve(5));
    }

    #[test]
    fn test_num_word_len() {
        let word_lens: HashMap<usize, usize> = NUM_WORD_LEN.iter().copied().collect();

        let ts = vec![
            (1, "one".len()),
            (9, "nine".len()),
            (10, "ten".len()),
            (14, "fourteen".len()),
            (20, "twenty".len()),
            (22, "twentytwo".len()),
            (100, "onehundred".len()),
            (101, "onehundredandone".len()),
            (121, "onehundredandtwentyone".len()),
        ];
        for (input, expected) in ts {
            assert_eq!(expected, num_word_len(input, &word_lens), "input={}", input);
        }
    }
}

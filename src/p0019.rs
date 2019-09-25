//! [Problem 19](https://projecteuler.net/problem=19)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2019))

pub struct Solver;

const FROM: (usize, usize) = (1901, 1);
const TO: (usize, usize) = (2000, 12);

impl super::Solver for Solver {
    fn solve(&self) -> i64 {
        solve(FROM, TO)
    }
}

fn solve((from_y, from_m): (usize, usize), (to_y, to_m): (usize, usize)) -> i64 {
    // 1900年1月1日が月曜日であるから、そこから7日ごとに月曜日に
    // なる。
    YearMonth::new(from_y, from_m)
        .take_while(|ym| ym.y < to_y || ym.y == to_y && ym.m <= to_m)
        .filter(|ym| ym.w == 0)
        .count() as i64
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct YearMonth {
    y: usize,
    m: usize,
    w: usize,
}

impl YearMonth {
    fn new(y: usize, m: usize) -> YearMonth {
        let diff = (1900..y).fold(0, |acc, y| acc + if is_leap_year(y) { 366 } else { 365 })
            + (1..m).fold(0, |acc, m| acc + days_in_month(y, m))
            + 1;
        YearMonth {
            y: y,
            m: m,
            w: diff % 7,
        }
    }
}

impl Iterator for YearMonth {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let res = *self;

        self.w = (self.w + days_in_month(self.y, self.m)) % 7;

        if self.m == 12 {
            self.y += 1;
            self.m = 1;
        } else {
            self.m += 1;
        };

        Some(res)
    }
}

fn is_leap_year(y: usize) -> bool {
    y % 400 == 0 || y % 4 == 0 && y % 100 != 0
}

fn days_in_month(y: usize, m: usize) -> usize {
    match m {
        2 if is_leap_year(y) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_year_month_new() {
        let ym = YearMonth::new(2000, 4);
        assert_eq!(2000, ym.y);
        assert_eq!(4, ym.m);
        assert_eq!(6, ym.w);
    }

    #[test]
    fn test_is_leap_year() {
        let ts = vec![
            (1900, false),
            (1901, false),
            (1902, false),
            (1903, false),
            (1904, true),
            (1905, false),
            (2000, true),
            (2001, false),
            (2002, false),
            (2003, false),
            (2004, true),
        ];
        for (y, expected) in ts {
            assert_eq!(expected, is_leap_year(y), "y={:?}", y);
        }
    }

    #[test]
    fn test_year_month_next() {
        let mut ym = YearMonth {
            y: 2000,
            m: 1,
            w: 6,
        };
        let ts = vec![
            (2000, 1, 6),
            (2000, 2, 2),
            (2000, 3, 3),
            (2000, 4, 6),
            (2000, 5, 1),
            (2000, 6, 4),
            (2000, 7, 6),
            (2000, 8, 2),
            (2000, 9, 5),
            (2000, 10, 0),
            (2000, 11, 3),
            (2000, 12, 5),
            (2001, 1, 1),
        ];
        for (expected_y, expected_m, expected_w) in ts {
            assert_eq!(
                Some(YearMonth {
                    y: expected_y,
                    m: expected_m,
                    w: expected_w
                }),
                ym.next(),
            );
        }
    }

    #[test]
    fn test_days_in_month() {
        let ts = vec![
            (1900, 1, 31),
            (1900, 2, 28),
            (1900, 3, 31),
            (1900, 4, 30),
            (1900, 5, 31),
            (1900, 6, 30),
            (1900, 7, 31),
            (1900, 8, 31),
            (1900, 9, 30),
            (1900, 10, 31),
            (1900, 11, 30),
            (1900, 12, 31),
            (1901, 1, 31),
            (1901, 2, 28),
            (1901, 3, 31),
            (1901, 4, 30),
            (1901, 5, 31),
            (1901, 6, 30),
            (1901, 7, 31),
            (1901, 8, 31),
            (1901, 9, 30),
            (1901, 10, 31),
            (1901, 11, 30),
            (1901, 12, 31),
            (2000, 1, 31),
            (2000, 2, 29),
            (2000, 3, 31),
            (2000, 4, 30),
            (2000, 5, 31),
            (2000, 6, 30),
            (2000, 7, 31),
            (2000, 8, 31),
            (2000, 9, 30),
            (2000, 10, 31),
            (2000, 11, 30),
            (2000, 12, 31),
        ];
        for (y, m, expected) in ts {
            assert_eq!(expected, days_in_month(y, m), "(y, m)={:?}", (y, m));
        }
    }

    #[test]
    fn test_solve() {
        assert_eq!(2, solve((2019, 1), (2019, 12)));
    }
}

//! [Problem 11](https://projecteuler.net/problem=11)([JP](http://www.odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2011))

pub struct Solver;

const SIZE: usize = 20;

const GRID: [[u64; SIZE]; SIZE] = [
    [
        08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08,
    ],
    [
        49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00,
    ],
    [
        81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65,
    ],
    [
        52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91,
    ],
    [
        22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80,
    ],
    [
        24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50,
    ],
    [
        32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70,
    ],
    [
        67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21,
    ],
    [
        24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72,
    ],
    [
        21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95,
    ],
    [
        78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92,
    ],
    [
        16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57,
    ],
    [
        86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58,
    ],
    [
        19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40,
    ],
    [
        04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66,
    ],
    [
        88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69,
    ],
    [
        04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36,
    ],
    [
        20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16,
    ],
    [
        20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54,
    ],
    [
        01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48,
    ],
];

const DISTS: [[(isize, isize); 4]; 4] = [
    [(0, 0), (0, 1), (0, 2), (0, 3)],    // 右方向
    [(0, 0), (1, 0), (2, 0), (3, 0)],    // 下方向
    [(0, 0), (1, 1), (2, 2), (3, 3)],    // 右下方向
    [(0, 0), (1, -1), (2, -2), (3, -3)], // 左下方向
];

impl super::Solver<u64> for Solver {
    fn solve(&self) -> u64 {
        solve()
    }
}

fn solve() -> u64 {
    (0..SIZE)
        .flat_map(|x| (0..SIZE).map(move |y| (x, y)))
        .fold(0, |acc, p| find_max_product(p).max(acc))
}

fn find_max_product(p: (usize, usize)) -> u64 {
    DISTS
        .iter()
        .filter_map(|dists| calc_product(p, dists))
        .max()
        .unwrap_or(0)
}

fn calc_product((x, y): (usize, usize), dists: &[(isize, isize); 4]) -> Option<u64> {
    let i = x as isize + dists[3].0;
    let j = y as isize + dists[3].1;
    if i < 0 || i >= SIZE as isize || j < 0 || j >= SIZE as isize {
        return None;
    }

    Some(
        dists
            .iter()
            .map(|(i, j)| GRID[(x as isize + i) as usize][(y as isize + j) as usize])
            .product(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_max_products() {
        let ts = vec![
            ((0, 0), 1651104),
            ((0, 17), 112112),
            ((17, 0), 2036880),
            ((19, 19), 0),
            ((6, 8), 4791800),
        ];
        for ((x, y), expected) in ts {
            assert_eq!(expected, find_max_product((x, y)))
        }
    }

    #[test]
    fn get_products() {
        let ts = vec![
            // (0, 0)
            (
                (0, 0),
                [(0, 0), (0, 1), (0, 2), (0, 3)],
                Some(GRID[0][0] * GRID[0][1] * GRID[0][2] * GRID[0][3]),
            ),
            (
                (0, 0),
                [(0, 0), (1, 0), (2, 0), (3, 0)],
                Some(GRID[0][0] * GRID[1][0] * GRID[2][0] * GRID[3][0]),
            ),
            (
                (0, 0),
                [(0, 0), (1, 1), (2, 2), (3, 3)],
                Some(GRID[0][0] * GRID[1][1] * GRID[2][2] * GRID[3][3]),
            ),
            ((0, 0), [(0, 0), (1, -1), (2, -2), (3, -3)], None),
            // (0, 17)
            ((0, 17), [(0, 0), (0, 1), (0, 2), (0, 3)], None),
            (
                (0, 17),
                [(0, 0), (1, 0), (2, 0), (3, 0)],
                Some(GRID[0][17] * GRID[1][17] * GRID[2][17] * GRID[3][17]),
            ),
            ((0, 17), [(0, 0), (1, 1), (2, 2), (3, 3)], None),
            (
                (0, 17),
                [(0, 0), (1, -1), (2, -2), (3, -3)],
                Some(GRID[0][17] * GRID[1][16] * GRID[2][15] * GRID[3][14]),
            ),
            // (17, 0)
            (
                (17, 0),
                [(0, 0), (0, 1), (0, 2), (0, 3)],
                Some(GRID[17][0] * GRID[17][1] * GRID[17][2] * GRID[17][3]),
            ),
            ((17, 0), [(0, 0), (1, 0), (2, 0), (3, 0)], None),
            ((17, 0), [(0, 0), (1, 1), (2, 2), (3, 3)], None),
            ((17, 0), [(0, 0), (1, -1), (2, -2), (3, -3)], None),
            // (19, 19)
            ((19, 19), [(0, 0), (0, 1), (0, 2), (0, 3)], None),
            ((19, 19), [(0, 0), (1, 0), (2, 0), (3, 0)], None),
            ((19, 19), [(0, 0), (1, 1), (2, 2), (3, 3)], None),
            ((19, 19), [(0, 0), (1, -1), (2, -2), (3, -3)], None),
            // (6, 8)
            (
                (6, 8),
                [(0, 0), (0, 1), (0, 2), (0, 3)],
                Some(GRID[6][8] * GRID[6][9] * GRID[6][10] * GRID[6][11]),
            ),
            (
                (6, 8),
                [(0, 0), (1, 0), (2, 0), (3, 0)],
                Some(GRID[6][8] * GRID[7][8] * GRID[8][8] * GRID[9][8]),
            ),
            (
                (6, 8),
                [(0, 0), (1, 1), (2, 2), (3, 3)],
                Some(GRID[6][8] * GRID[7][9] * GRID[8][10] * GRID[9][11]),
            ),
            (
                (6, 8),
                [(0, 0), (1, -1), (2, -2), (3, -3)],
                Some(GRID[6][7] * GRID[7][6] * GRID[8][5] * GRID[9][5]),
            ),
        ];
        for (p, dists, expected) in ts {
            assert_eq!(expected, calc_product(p, &dists), "{:?} + {:?}", p, dists)
        }
    }
}
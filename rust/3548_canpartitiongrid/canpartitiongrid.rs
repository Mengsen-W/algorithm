struct Solution;

use std::collections::HashSet;

impl Solution {
    fn rotation(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut tmp = vec![vec![0; m]; n];

        for i in 0..m {
            for j in 0..n {
                tmp[j][m - 1 - i] = grid[i][j];
            }
        }
        tmp
    }

    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let mut grid = grid;
        let mut total: i64 = 0;
        let mut sum: i64;
        let mut tag: i64;
        let mut m = grid.len();
        let mut n = grid[0].len();
        for i in 0..m {
            for j in 0..n {
                total += grid[i][j] as i64;
            }
        }

        let mut exist = HashSet::new();

        for _ in 0..4 {
            exist.clear();
            exist.insert(0);
            sum = 0;
            m = grid.len();
            n = grid[0].len();
            if m < 2 {
                grid = Self::rotation(&grid);
                continue;
            }
            if n == 1 {
                for i in 0..m - 1 {
                    sum += grid[i][0] as i64;
                    tag = sum * 2 - total;
                    if tag == 0 || tag == grid[0][0] as i64 || tag == grid[i][0] as i64 {
                        return true;
                    }
                }
                grid = Self::rotation(&grid);
                continue;
            }

            for i in 0..m - 1 {
                for j in 0..n {
                    exist.insert(grid[i][j] as i64);
                    sum += grid[i][j] as i64;
                }

                tag = sum * 2 - total;

                if i == 0 {
                    if tag == 0 || tag == grid[0][0] as i64 || tag == grid[0][n - 1] as i64 {
                        return true;
                    }
                    continue;
                }

                if exist.contains(&tag) {
                    return true;
                }
            }

            grid = Self::rotation(&grid);
        }

        false
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 4], vec![2, 3]], true),
        (vec![vec![1, 2], vec![3, 4]], true),
        (vec![vec![1, 2, 4], vec![2, 3, 5]], false),
        (vec![vec![4, 1, 8], vec![3, 2, 6]], false),
    ];

    for (test, expected) in tests {
        assert_eq!(Solution::can_partition_grid(test), expected);
    }
}

struct Solution;

impl Solution {
    pub fn can_partition_grid(mut grid: Vec<Vec<i32>>) -> bool {
        let mut total: i64 = 0;
        let mut m = grid.len();
        let mut n = grid[0].len();
        for i in 0..m {
            for j in 0..n {
                total += grid[i][j] as i64;
            }
        }
        for _ in 0..2 {
            let mut sum: i64 = 0;
            m = grid.len();
            n = grid[0].len();
            for i in 0..m - 1 {
                for j in 0..n {
                    sum += grid[i][j] as i64;
                }
                if sum * 2 == total {
                    return true;
                }
            }
            grid = Self::rotation(grid);
        }
        false
    }

    fn rotation(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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
}

fn main() {
    let tests = vec![
        (vec![vec![1, 4], vec![2, 3]], true),
        (vec![vec![1, 3], vec![2, 4]], false),
    ];

    for (grid, expected) in tests {
        assert_eq!(Solution::can_partition_grid(grid), expected);
    }
}

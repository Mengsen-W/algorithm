struct Solution;

impl Solution {
    pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut cols = vec![0; m];
        let mut res = 0;

        for i in 0..n {
            let mut row_sum = 0;
            for j in 0..m {
                cols[j] += grid[i][j];
                row_sum += cols[j];
                if row_sum <= k {
                    res += 1;
                }
            }
        }

        res
    }
}

fn main() {
    let tests = vec![
        (vec![vec![7, 6, 3], vec![6, 6, 1]], 18, 4),
        (vec![vec![7, 2, 9], vec![1, 5, 0], vec![2, 6, 6]], 20, 6),
    ];

    for (grid, k, expected) in tests {
        assert_eq!(Solution::count_submatrices(grid, k), expected);
    }
}

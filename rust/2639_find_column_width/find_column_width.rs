/*
 * @Date: 2024-04-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-27
 * @FilePath: /algorithm/rust/2639_find_column_width/find_column_width.rs
 */

struct Solution;

impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let m = grid[0].len();
        let mut res = vec![0; m];
        for i in 0..n {
            for j in 0..m {
                let mut x = grid[i][j];
                let mut length = 0;
                if x <= 0 {
                    length = 1;
                }
                while x != 0 {
                    length += 1;
                    x /= 10;
                }
                res[j] = res[j].max(length);
            }
        }
        return res;
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1], vec![22], vec![333]], vec![3]),
        (
            vec![vec![-15, 1, 3], vec![15, 7, 12], vec![5, 6, -2]],
            vec![3, 1, 2],
        ),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::find_column_width(grid), ans);
    }
}

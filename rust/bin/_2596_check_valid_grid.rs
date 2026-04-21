/*
 * @Date: 2023-09-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-13
 * @FilePath: /algorithm/rust/2596_check_valid_grid/check_valid_grid.rs
 */

struct Solution;
impl Solution {
    pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        let mut d = vec![(0_usize, 0_usize); n * n];
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 0 {
                    if i != 0 || j != 0 {
                        return false;
                    }
                }
                d[grid[i][j] as usize] = (i, j);
            }
        }
        for i in 1..n * n {
            let a = (d[i].0 as i32) - (d[i - 1].0 as i32);
            let b = (d[i].1 as i32) - (d[i - 1].1 as i32);
            if a * a + b * b != 5 {
                return false;
            }
        }
        true
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec![0, 11, 16, 5, 20],
                vec![17, 4, 19, 10, 15],
                vec![12, 1, 8, 21, 6],
                vec![3, 18, 23, 14, 9],
                vec![24, 13, 2, 7, 22],
            ],
            true,
        ),
        (vec![vec![0, 3, 6], vec![5, 8, 1], vec![2, 7, 4]], false),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::check_valid_grid(grid), ans);
    }
}

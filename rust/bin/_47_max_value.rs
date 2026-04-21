/*
 * @Date: 2023-03-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-08
 * @FilePath: /algorithm/rust/47_max_value/max_value.rs
 */

pub fn max_value(mut grid: Vec<Vec<i32>>) -> i32 {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            grid[i][j] +=
                (if i > 0 { grid[i - 1][j] } else { 0 }).max(if j > 0 { grid[i][j - 1] } else { 0 })
        }
    }
    grid[grid.len() - 1][grid[0].len() - 1]
}

fn main() {
    let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    let ans = 12;
    assert_eq!(max_value(grid), ans);
}

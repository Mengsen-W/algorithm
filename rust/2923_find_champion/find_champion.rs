/*
 * @Date: 2024-04-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-12
 * @FilePath: /algorithm/rust/2923_find_champion/find_champion.rs
 */

struct Solution;

impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        for i in 0..n {
            let sum: i32 = grid[i].iter().sum();
            if sum == n as i32 - 1 {
                return i as i32;
            }
        }
        -1
    }
}

fn main() {
    let tests = vec![
        (vec![vec![0, 1], vec![0, 0]], 0),
        (vec![vec![0, 0, 1], vec![1, 0, 1], vec![0, 0, 0]], 1),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::find_champion(grid), ans);
    }
}

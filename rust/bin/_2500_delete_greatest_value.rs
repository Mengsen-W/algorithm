/*
 * @Date: 2023-07-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-27
 * @FilePath: /algorithm/rust/2500_delete_greatest_value/delete_greatest_value.rs
 */

struct Solution;
impl Solution {
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
        for v in grid.iter_mut() {
            v.sort_unstable_by(|x, y| y.cmp(x));
        }
        let n = grid.first().unwrap().len(); // let n = grid[0].len();
        let mut res = 0;
        for j in 0..n {
            res += grid.iter().map(|row| row[j]).max().unwrap();
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 2, 4], vec![3, 3, 1]], 8),
        (vec![vec![10]], 10),
    ];
    for (grid, ans) in tests {
        assert_eq!(Solution::delete_greatest_value(grid), ans);
    }
}

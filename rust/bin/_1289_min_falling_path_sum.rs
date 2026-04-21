/*
 * @Date: 2023-08-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-10
 * @FilePath: /algorithm/rust/1289_min_falling_path_sum/min_falling_path_sum.rs
 */

struct Solution;
impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut rcd = grid[0].clone();
        for row in grid.iter().skip(1) {
            let mut new_rcd = Vec::new();
            let (min_idx, min_val) = rcd.iter().enumerate().min_by(|a, b| a.1.cmp(b.1)).unwrap();
            new_rcd.reserve_exact(rcd.len());
            for (idx, v) in row.iter().enumerate() {
                if idx != min_idx {
                    new_rcd.push(min_val + v);
                } else {
                    let m = rcd
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != idx)
                        .map(|x| x.1)
                        .min();
                    if let Some(m) = m {
                        new_rcd.push(m + v);
                    }
                }
            }
            rcd = new_rcd;
        }
        rcd.into_iter().min().unwrap()
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 13),
        (vec![vec![7]], 7),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::min_falling_path_sum(grid), ans);
    }
}

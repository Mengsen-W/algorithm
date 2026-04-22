/*
 * @Date: 2023-07-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-13
 * @FilePath: /algorithm/rust/931_min_falling_path_sum/min_falling_path_sum.rs
 */

struct Solution;
impl Solution {
    pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        for i in 1..m {
            (0..n).for_each(|j| {
                matrix[i][j] += *matrix[i - 1][(j.max(1) - 1)..(j.min(n - 2) + 2)]
                    .iter()
                    .min()
                    .unwrap()
            });
        }
        *matrix[m - 1].iter().min().unwrap()
    }
}

fn main() {
    let tests = vec![
        (vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]], 13),
        (vec![vec![-19, 57], vec![-40, -5]], -59),
    ];

    for (matrix, ans) in tests {
        assert_eq!(Solution::min_falling_path_sum(matrix), ans);
    }
}

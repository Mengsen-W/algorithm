/*
 * @Date: 2022-03-27 02:45:02
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-27
 * @FilePath: /algorithm/rust/2028_missing_rolls/missing_rolls.rs
 */

struct Solution;

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let sum = (rolls.len() as i32 + n) * mean - rolls.iter().sum::<i32>();
        if sum < n || sum > 6 * n {
            return vec![];
        }
        [
            vec![sum / n; (n - sum % n) as usize],
            vec![sum / n + 1; (sum % n) as usize],
        ]
        .concat()
    }
}

fn main() {
    let tests = vec![
        (vec![3, 2, 4, 3], 4, 2, vec![6, 6]),
        (vec![1, 5, 6], 3, 4, vec![2, 2, 2, 3]),
        (vec![1, 2, 3, 4], 5, 4, vec![]),
        (vec![1], 3, 1, vec![5]),
    ];

    for (rolls, mean, n, ans) in tests {
        assert_eq!(Solution::missing_rolls(rolls, mean, n), ans);
    }
}

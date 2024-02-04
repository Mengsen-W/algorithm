/*
 * @Date: 2024-02-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-04
 * @FilePath: /algorithm/rust/1690_window.native/stone_game_vii.rs
 */

struct Solution;

impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut s = vec![0; n + 1];
        for (i, &x) in stones.iter().enumerate() {
            s[i + 1] = s[i] + x;
        }
        let mut f = vec![0; n];
        for i in (0..n - 1).rev() {
            for j in i + 1..n {
                f[j] = (s[j + 1] - s[i + 1] - f[j]).max(s[j] - s[i] - f[j - 1]);
            }
        }
        f[n - 1]
    }
}

fn main() {
    let tests = vec![
        (vec![5, 3, 1, 4, 2], 6),
        (vec![7, 90, 5, 1, 100, 10, 10, 2], 122),
    ];

    for (stones, ans) in tests {
        assert_eq!(Solution::stone_game_vii(stones), ans);
    }
}

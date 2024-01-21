/*
 * @Date: 2024-01-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-21
 * @FilePath: /algorithm/rust/410_split_array/split_array.rs
 */

struct Solution;

impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let n = nums.len();
        let mut f = vec![vec![std::i32::MAX; m as usize + 1]; n + 1];
        let mut sub = vec![0; n + 1];

        for i in 0..n {
            sub[i + 1] = sub[i] + nums[i];
        }
        f[0][0] = 0;
        for i in 1..=n {
            for j in 1..=i.min(m as usize) {
                for k in 0..i {
                    f[i][j] = f[i][j].min(f[k][j - 1].max(sub[i] - sub[k]));
                }
            }
        }
        f[n][m as usize]
    }
}

fn main() {
    let tests = vec![
        (vec![7, 2, 5, 10, 8], 2, 18),
        (vec![1, 2, 3, 4, 5], 2, 9),
        (vec![1, 4, 4], 3, 4),
    ];

    for (nums, m, ans) in tests {
        assert_eq!(Solution::split_array(nums, m), ans);
    }
}

/*
 * @Date: 2023-10-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-10
 * @FilePath: /algorithm/rust/2731_sum_distance/sum_distance.rs
 */

struct Solution;

impl Solution {
    pub fn sum_distance(nums: Vec<i32>, s: String, d: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut a = vec![0; nums.len()];
        let s = s.as_bytes();
        for (i, &x) in nums.iter().enumerate() {
            let d = if s[i] == 'L' as u8 { -d } else { d };
            a[i] = x as i64 + d as i64;
        }
        a.sort();

        let mut ans = 0i64;
        let mut sum = 0i64;
        for (i, &x) in a.iter().enumerate() {
            ans = (ans + i as i64 * x - sum) % MOD;
            sum += x;
        }
        ans as i32
    }
}

fn main() {
    let tests = vec![(vec![-2, 0, 2], "RLL", 3, 8), (vec![1, 0], "RL", 2, 5)];

    for (nums, s, d, ans) in tests {
        assert_eq!(Solution::sum_distance(nums, s.to_string(), d), ans);
    }
}

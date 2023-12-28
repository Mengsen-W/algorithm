/*
 * @Date: 2023-12-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-28
 * @FilePath: /algorithm/rust/2735_min_cost/min_cost.rs
 */

struct Solution;

impl Solution {
    pub fn min_cost(nums: Vec<i32>, x: i32) -> i64 {
        let n = nums.len();
        let mut s: Vec<i64> = (0..n).map(|i| i as i64 * x as i64).collect();
        for i in 0..n {
            let mut mn = nums[i];
            for j in i..(n + i) {
                mn = mn.min(nums[j % n]);
                s[j - i] += mn as i64;
            }
        }
        *s.iter().min().unwrap()
    }
}

fn main() {
    let tests = vec![(vec![20, 1, 15], 5, 13), (vec![1, 2, 3], 4, 6)];

    for (nums, x, ans) in tests {
        assert_eq!(Solution::min_cost(nums, x), ans);
    }
}

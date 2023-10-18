/*
 * @Date: 2023-10-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-18
 * @FilePath: /algorithm/rust/2530_max_kelements/max_kelements.rs
 */

struct Solution;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        use std::collections::BinaryHeap;
        let mut h = BinaryHeap::from(nums);
        let mut ans = 0i64;
        for _ in 0..k {
            let mx = h.pop().unwrap();
            ans += mx as i64;
            h.push((mx + 2) / 3);
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![10, 10, 10, 10, 10], 5, 50),
        (vec![1, 10, 3, 3, 3], 3, 17),
    ];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::max_kelements(nums, k), ans);
    }
}

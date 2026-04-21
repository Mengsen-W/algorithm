/*
 * @Date: 2024-02-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-05
 * @FilePath: /algorithm/rust/1696_max_result/max_result.rs
 */

struct Solution;

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::VecDeque;
        let n = nums.len();
        let k = k as usize;
        let mut f = vec![0; n];
        f[0] = nums[0];
        let mut q = VecDeque::new();
        q.push_back(0);
        for i in 1..n {
            // 1. 出
            if *q.front().unwrap() + k < i {
                q.pop_front();
            }
            // 2. 转移
            f[i] = f[*q.front().unwrap()] + nums[i];
            // 3. 入
            while !q.is_empty() && f[i] >= f[*q.back().unwrap()] {
                q.pop_back();
            }
            q.push_back(i);
        }
        f[n - 1]
    }
}

fn main() {
    let tests = vec![
        (vec![1, -1, -2, 4, -7, 3], 2, 7),
        (vec![10, -5, -2, 4, 0, 3], 3, 17),
        (vec![1, -5, -20, 4, -1, 3, -6, -3], 2, 0),
    ];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::max_result(nums, k), ans);
    }
}

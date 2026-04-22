/*
 * @Date: 2024-05-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-23
 * @FilePath: /algorithm/rust/2831_longest_equal_subarray/longest_equal_subarray.rs
 */

struct Solution;

impl Solution {
    pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut pos: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            pos.entry(num).or_insert(Vec::new()).push(i);
        }
        let mut ans = 0;
        for vec in pos.values() {
            let mut j = 0;
            for i in 0..vec.len() {
                /* 缩小窗口，直到不同元素数量小于等于 k */
                while vec[i] as i32 - vec[j] as i32 - (i as i32 - j as i32) > k {
                    j += 1;
                }
                ans = ans.max(i - j + 1);
            }
        }
        ans as i32
    }
}

fn main() {
    let tests = vec![
        (vec![1, 3, 2, 3, 1, 3], 3, 3),
        (vec![1, 1, 2, 2, 1, 1], 2, 4),
    ];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::longest_equal_subarray(nums, k), ans);
    }
}

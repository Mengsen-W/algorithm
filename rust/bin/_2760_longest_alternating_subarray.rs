/*
 * @Date: 2023-11-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-16
 * @FilePath: /algorithm/rust/2760_longest_alternating_subarray/longest_alternating_subarray.rs
 */

struct Solution;

impl Solution {
    pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        let mut i = 0;
        while i < n {
            if nums[i] > threshold || nums[i] % 2 != 0 {
                i += 1; // 直接跳过
                continue;
            }
            let start = i; // 记录这一组的开始位置
            i += 1; // 开始位置已经满足要求，从下一个位置开始判断
            while i < n && nums[i] <= threshold && nums[i] % 2 != nums[i - 1] % 2 {
                i += 1;
            }
            // 从 start 到 i-1 是满足题目要求的子数组
            ans = ans.max(i - start);
        }
        ans as i32
    }
}

fn main() {
    let tests = vec![
        (vec![3, 2, 5, 4], 5, 3),
        (vec![1, 2], 2, 1),
        (vec![2, 3, 4, 5], 4, 3),
    ];

    for (nums, threshold, ans) in tests {
        assert_eq!(Solution::longest_alternating_subarray(nums, threshold), ans);
    }
}

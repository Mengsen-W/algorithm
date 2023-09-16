/*
 * @Date: 2023-09-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-16
 * @FilePath: /algorithm/rust/198_rob/rob.rs
 */

struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        nums.iter().fold((0, 0), |(a, b), x| (b, b.max(a + x))).1
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 3, 1], 4), (vec![2, 7, 9, 3, 1], 12)];
    for (nums, ans) in tests {
        assert_eq!(Solution::rob(nums), ans);
    }
}

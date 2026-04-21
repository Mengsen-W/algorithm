/*
 * @Date: 2023-10-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-14
 * @FilePath: /algorithm/rust/136_single_number/single_number.rs
 */

struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |xor, &x| xor ^ x)
    }
}

fn main() {
    let tests = vec![(vec![2, 2, 1], 1), (vec![4, 1, 2, 1, 2], 4), (vec![1], 1)];
    for (nums, ans) in tests {
        assert_eq!(Solution::single_number(nums), ans);
    }
}

/*
 * @Date: 2021-11-06 01:09:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-06 01:23:12
 */

struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let total = (n as i32) * (n as i32 + 1) / 2;
        let mut arr_sum = 0;
        for i in nums {
            arr_sum += i;
        }
        total - arr_sum
    }
}

fn main() {
    assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    assert_eq!(Solution::missing_number(vec![0, 1]), 2);
    assert_eq!(Solution::missing_number(vec![0]), 1);
}

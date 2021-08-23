/*
 * @Date: 2021-08-23 21:10:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-23 21:21:57
 */

struct Solution;

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        let n = n as usize;
        if n == 0 {
            return 0;
        }
        let mut nums: Vec<i32> = vec![0; n + 1];
        nums[1] = 1;
        for i in 2..=n {
            nums[i] = nums[i / 2] + i as i32 % 2 * nums[i / 2 + 1];
        }
        *nums.iter().max().unwrap()
    }
}

fn main() {
  assert_eq!(Solution::get_maximum_generated(7), 3);
  assert_eq!(Solution::get_maximum_generated(2), 1);
  assert_eq!(Solution::get_maximum_generated(3), 2);
}

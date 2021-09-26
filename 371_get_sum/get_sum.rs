/*
 * @Date: 2021-09-26 08:52:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-26 09:20:57
 */

struct Solution;

impl Solution {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let carry = ((a & b) as u32) << 1;
            a = a ^ b;
            b = carry as i32;
        }
        a
    }
}

fn main() {
    assert_eq!(Solution::get_sum(10, 2), 12);
    assert_eq!(Solution::get_sum(0, 2), 2);
}

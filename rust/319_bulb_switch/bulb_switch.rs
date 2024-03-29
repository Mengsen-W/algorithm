/*
 * @Date: 2021-11-15 01:29:50
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-15 01:38:36
 */

struct Solution;

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64).sqrt() as i32
    }
}

fn main() {
    assert_eq!(Solution::bulb_switch(0), 0);
    assert_eq!(Solution::bulb_switch(1), 1);
    assert_eq!(Solution::bulb_switch(3), 1);
}

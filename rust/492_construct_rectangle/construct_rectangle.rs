/*
 * @Date: 2021-10-23 02:11:24
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-23 02:25:01
 */

struct Solution;

impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut w = (area as f64).sqrt() as i32;
        while area % w != 0 {
            w -= 1;
        }
        vec![area / w, w]
    }
}

fn main() {
    assert_eq!(Solution::construct_rectangle(4), vec![2, 2]);
}

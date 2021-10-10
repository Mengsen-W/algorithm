/*
 * @Date: 2021-10-10 09:26:26
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-10 09:45:16
 */

struct Solution;

impl Solution {
    // pub fn arrange_coins(n: i32) -> i32 {
    //     let mut i = 1;
    //     let mut j = 65536;
    //     while i < j {
    //         let m = (i + j) >> 1;
    //         let sum = ((1 + m as i64) * m as i64) >> 1;
    //         let diff = n - sum as i32;
    //         if diff >= m + 1 {
    //             i = m + 1;
    //         } else {
    //             j = m;
    //         }
    //     }
    //     i
    // }
    pub fn arrange_coins(n: i32) -> i32 {
        ((0.25 + 2.0 * (n as f64)).sqrt() - 0.5) as i32
    }
}

fn main() {
    assert_eq!(Solution::arrange_coins(5), 2);
    assert_eq!(Solution::arrange_coins(8), 3);
}

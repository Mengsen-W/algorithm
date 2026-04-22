/*
 * @Date: 2021-10-18 08:54:12
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-18 09:08:21
 */

struct Solution;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        // let mut highbit = 0;
        // for i in 1..=30 {
        //     if num >= (1 << i) {
        //         highbit = i;
        //     } else {
        //         break;
        //     }
        // }
        // let mask = match highbit {
        //     h if h == 30 => 0x7fffffff,
        //     _ => (1 << (highbit + 1)) - 1,
        // };
        // num ^ mask
        (1 << (i32::BITS - num.leading_zeros())) - 1 - num
    }
}

fn main() {
    assert_eq!(Solution::find_complement(5), 2);
    assert_eq!(Solution::find_complement(1), 0);
}

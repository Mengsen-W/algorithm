/*
 * @Date: 2021-11-07 02:02:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-07 02:15:35
 */

struct Solution;

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let (mut mina, mut minb) = (m, n);
        for i in &ops {
            mina = mina.min(i[0]);
            minb = minb.min(i[1]);
        }
        mina * minb
    }
}

fn main() {
    assert_eq!(Solution::max_count(3, 3, vec![vec![2, 2], vec![3, 3]]), 4);
}

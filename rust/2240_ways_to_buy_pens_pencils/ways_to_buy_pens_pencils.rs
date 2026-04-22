/*
 * @Date: 2023-09-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-01
 * @FilePath: /algorithm/rust/2240_ways_to_buy_pens_pencils/ways_to_buy_pens_pencils.rs
 */

struct Solution;
impl Solution {
    pub fn ways_to_buy_pens_pencils(total: i32, mut cost1: i32, mut cost2: i32) -> i64 {
        if cost1 < cost2 {
            std::mem::swap(&mut cost1, &mut cost2);
        }
        (0..=total / cost1).fold(0, |res, i| res + ((total - i * cost1) / cost2) as i64 + 1)
    }
}

fn main() {
    let tests = vec![(20, 10, 5, 9), (5, 10, 10, 1)];

    for (total, cost1, cost2, ans) in tests {
        assert_eq!(Solution::ways_to_buy_pens_pencils(total, cost1, cost2), ans);
    }
}

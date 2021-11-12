/*
 * @Date: 2021-11-12 01:08:08
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-12 01:23:40
 */

struct Solution;

impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        let mut f: Vec<Vec<i32>> = vec![vec![0; n + 1]; n + 1];
        for i in (1..=n - 1).rev() {
            for j in i + 1..=n {
                let mut min_cost = i32::MAX;
                for k in i..j {
                    let cost = k as i32 + std::cmp::max(f[i][k - 1], f[k + 1][j]);
                    min_cost = std::cmp::min(min_cost, cost);
                }
                f[i][j] = min_cost;
            }
        }
        f[1][n]
    }
}

fn main() {
    assert_eq!(Solution::get_money_amount(10), 16);
    assert_eq!(Solution::get_money_amount(2), 1);
    assert_eq!(Solution::get_money_amount(1), 0);
}

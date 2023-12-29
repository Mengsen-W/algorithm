/*
 * @Date: 2023-12-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-29
 * @FilePath: /algorithm/rust/2706_buy_choco/buy_choco.rs
 */

struct Solution;

impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut a = 1000;
        let mut b = 1000;
        for &x in prices.iter() {
            if x < a {
                b = a;
                a = x;
            } else if x < b {
                b = x;
            }
        }
        let cost = a + b;
        if money < cost {
            money
        } else {
            money - cost
        }
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 2], 3, 0), (vec![3, 2, 3], 3, 3)];

    for (prices, money, ans) in tests {
        assert_eq!(Solution::buy_choco(prices, money), ans);
    }
}

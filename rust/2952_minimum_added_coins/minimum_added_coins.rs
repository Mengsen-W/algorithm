/*
 * @Date: 2024-03-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-30
 * @FilePath: /algorithm/rust/2952_minimum_added_coins/minimum_added_coins.rs
 */

struct Solution;

impl Solution {
    pub fn minimum_added_coins(mut coins: Vec<i32>, target: i32) -> i32 {
        coins.sort_unstable();
        let mut ans = 0;
        let mut s = 1;
        let mut i = 0;
        while s <= target {
            if i < coins.len() && coins[i] <= s {
                s += coins[i];
                i += 1;
            } else {
                s *= 2; // 必须添加 s
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![1, 4, 10], 19, 2),
        (vec![1, 4, 10, 5, 7, 19], 19, 1),
        (vec![1, 1, 1], 20, 3),
    ];

    for (coins, target, ans) in tests {
        assert_eq!(Solution::minimum_added_coins(coins, target), ans);
    }
}

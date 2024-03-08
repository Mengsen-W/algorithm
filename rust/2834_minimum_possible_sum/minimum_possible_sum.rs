/*
 * @Date: 2024-03-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-08
 * @FilePath: /algorithm/rust/2834_minimum_possible_sum/minimum_possible_sum.rs
 */

struct Solution;

impl Solution {
    pub fn minimum_possible_sum(n: i32, target: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n: i64 = n as i64;
        let target: i64 = target as i64;
        let m: i64 = target / 2;
        if n <= m {
            return (((1 + n) * n / 2) % MOD) as i32;
        }
        return ((((1 + m) * m / 2) + ((target + target + (n - m) - 1) * (n - m) / 2)) % MOD)
            as i32;
    }
}

fn main() {
    let tests = vec![(2, 3, 4), (3, 3, 8), (1, 1, 1)];

    for (n, target, ans) in tests {
        assert_eq!(Solution::minimum_possible_sum(n, target), ans);
    }
}

/*
 * @Date: 2023-10-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-24
 * @FilePath: /algorithm/rust/1155_num_rolls_to_target/num_rolls_to_target.rs
 */

struct Solution;

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        if target < n || target > n * k {
            return 0; // 无法组成 target
        }
        const MOD: i64 = 1_000_000_007;
        let (n, k, target) = (n as usize, k as usize, target as usize);
        let mut f = vec![0i64; target - n + 1];
        f[0] = 1;
        for i in 1..=n {
            let max_j = (i * (k - 1)).min(target - n); // i 个骰子至多掷出 i*(k-1)
            for j in 1..=max_j {
                f[j] += f[j - 1]; // 原地计算 f 的前缀和
            }
            for j in (k..=max_j).rev() {
                f[j] = (f[j] - f[j - k]) % MOD; // f[j] 是两个前缀和的差
            }
        }
        f[target - n] as i32
    }
}

fn main() {
    let tests = vec![(1, 6, 3, 1), (2, 6, 7, 6), (30, 30, 500, 222616187)];
    for (n, k, target, ans) in tests {
        assert_eq!(Solution::num_rolls_to_target(n, k, target), ans);
    }
}

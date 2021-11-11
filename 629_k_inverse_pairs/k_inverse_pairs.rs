/*
 * @Date: 2021-11-11 01:11:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-11 01:23:59
 */

struct Solution;

impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let k = k as usize;
        let n = n as usize;
        let mut f: Vec<Vec<i32>> = vec![vec![0; k as usize + 1]; 2];
        f[0][0] = 1;
        for i in 1..=n {
            for j in 0..=k {
                let cur = i & 1;
                let prev = cur ^ 1;
                f[cur][j] = 0;
                if j > 0 {
                    f[cur][j] = f[cur][j - 1];
                }
                if j >= i {
                    f[cur][j] -= f[prev][j - i];
                }
                f[cur][j] += f[prev][j];
                if f[cur][j] >= MOD {
                    f[cur][j] -= MOD;
                } else if f[cur][j] < 0 {
                    f[cur][j] += MOD;
                }
            }
        }
        f[n & 1][k]
    }
}

fn main() {
    assert_eq!(Solution::k_inverse_pairs(3, 0), 1);
    assert_eq!(Solution::k_inverse_pairs(3, 1), 2);
}

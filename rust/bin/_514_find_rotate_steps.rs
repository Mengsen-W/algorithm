/*
 * @Date: 2024-01-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-29
 * @FilePath: /algorithm/rust/514_find_rotate_steps/find_rotate_steps.rs
 */

struct Solution;

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let (n, m) = (ring.len(), key.len());
        let (ring, key) = (
            ring.chars().collect::<Vec<_>>(),
            key.chars().collect::<Vec<_>>(),
        );
        let mut pos = vec![vec![]; 26];
        for (i, &c) in ring.iter().enumerate() {
            pos[(c as u8 - b'a') as usize].push(i);
        }
        let mut dp = vec![vec![std::i32::MAX; n]; m];

        for &i in pos[(key[0] as u8 - b'a') as usize].iter() {
            dp[0][i] = i.min(n - i) as i32 + 1;
        }

        for i in 1..m {
            for &j in pos[(key[i] as u8 - b'a') as usize].iter() {
                for &k in pos[(key[i - 1] as u8 - b'a') as usize].iter() {
                    dp[i][j] = std::cmp::min(
                        dp[i][j],
                        dp[i - 1][k] + std::cmp::min((j + n - k) % n, (k + n - j) % n) as i32 + 1,
                    );
                }
            }
        }
        *dp[m - 1].iter().min().unwrap()
    }
}

fn main() {
    let tests = vec![("godding", "gd", 4), ("godding", "godding", 13)];

    for (ring, key, ans) in tests {
        assert_eq!(
            Solution::find_rotate_steps(ring.to_string(), key.to_string()),
            ans
        );
    }
}

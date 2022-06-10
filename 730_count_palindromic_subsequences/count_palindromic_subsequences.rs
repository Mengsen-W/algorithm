/*
 * @Date: 2022-06-10 09:55:01
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-10 10:25:39
 * @FilePath: /algorithm/730_count_palindromic_subsequences/count_palindromic_subsequences.rs
 */

pub fn count_palindromic_subsequences(s: String) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    let mut dp = vec![vec![0_i64; n]; n];
    for i in 0..n {
        dp[i][i] = 1;
    }

    for len in 2..=n {
        for i in 0..=(n - len) {
            let j = i + len - 1;
            if s[i] == s[j] {
                let mut low = i + 1;
                let mut high = j - 1;
                while low <= high && s[low] != s[i] {
                    low += 1;
                }
                while high >= low && s[high] != s[j] {
                    high -= 1;
                }
                if low > high {
                    dp[i][j] = (2 + dp[i + 1][j - 1] * 2) % MOD;
                } else if low == high {
                    dp[i][j] = (1 + dp[i + 1][j - 1] * 2) % MOD;
                } else {
                    dp[i][j] = (0 + dp[i + 1][j - 1] * 2 - dp[low + 1][high - 1] + MOD) % MOD;
                }
            } else {
                dp[i][j] = (0 + dp[i + 1][j] + dp[i][j - 1] - dp[i + 1][j - 1] + MOD) % MOD;
            }
        }
    }
    dp[0][n - 1] as i32
}

fn main() {
    assert_eq!(count_palindromic_subsequences(String::from("bccb")), 6);
    assert_eq!(
        count_palindromic_subsequences(String::from(
            "abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba"
        )),
        104860361
    );
}

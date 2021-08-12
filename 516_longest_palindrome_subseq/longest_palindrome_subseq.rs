/*
 * @Date: 2021-08-12 14:18:40
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-12 15:21:47
 */

struct Solution;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut dp = vec![vec![0; n]; n];
        for i in (0..=n - 1).rev() {
            dp[i][i] = 1;
            let c1 = s[i];
            for j in i + 1..n {
                let c2 = s[j];
                match c1 == c2 {
                    true => dp[i][j] = dp[i + 1][j - 1] + 2,
                    false => dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]),
                };
            }
        }
        dp[0][n - 1]
    }
}

fn main() {
    {
        let s = "bbbab".to_string();
        assert_eq!(Solution::longest_palindrome_subseq(s), 4);
    }
    {
        let s = "cbbd".to_string();
        assert_eq!(Solution::longest_palindrome_subseq(s), 2);
    }
}

struct Solution;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        use std::cmp::min;
        let mut is_palindrome = vec![vec![true; s.len() + 1]; s.len()];
        for r in 2..s.len() + 1 {
            for l in (0..r - 1).rev() {
                is_palindrome[l][r] =
                    s.as_bytes()[l] == s.as_bytes()[r - 1] && is_palindrome[l + 1][r - 1];
            }
        }

        let mut dp = vec![0; s.len() + 1];
        for r in 1..s.len() + 1 {
            dp[r] = r as i32;
            for l in 0..r {
                if is_palindrome[l][r] {
                    dp[r] = min(dp[r], dp[l] + 1);
                }
            }
        }
        return dp[s.len()] - 1;
    }
}

fn main() {
    let tests = vec![("aab", 1), ("a", 0), ("ab", 1)];

    for (s, ans) in tests {
        assert_eq!(Solution::min_cut(s.to_string()), ans);
    }
}

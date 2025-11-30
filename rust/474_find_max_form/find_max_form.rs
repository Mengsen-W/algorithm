/*
 * @Date: 2021-06-06 09:36:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-06 10:04:19
 */

struct Solution;

impl Solution {
    fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let n_usize = n as usize;
        let m_usize = m as usize;
        let mut dp = vec![vec![0; n_usize + 1]; m_usize + 1];

        for s in strs {
            let zeros = s.chars().filter(|&c| c == '0').count();
            let ones = s.chars().filter(|&c| c == '1').count();
            for i in (zeros..=m_usize).rev() {
                for j in (ones..=n_usize).rev() {
                    dp[i][j] = dp[i][j].max(1 + dp[i - zeros][j - ones]);
                }
            }
        }

        dp[m_usize][n_usize]
    }
}

fn main() {
    let tests = vec![
        (vec!["10", "0001", "111001", "1", "0"], 5, 3, 4),
        (vec!["01", "0", "1"], 1, 1, 2),
    ];

    for (strs, m, n, expected) in tests {
        assert_eq!(
            Solution::find_max_form(strs.iter().map(|s| s.to_string()).collect(), m, n),
            expected
        );
    }
}

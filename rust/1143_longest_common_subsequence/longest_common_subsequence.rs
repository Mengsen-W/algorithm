/*
 * @Date: 2021-04-04 19:01:23
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-04 19:15:06
 */

#[allow(non_snake_case)]
fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let N = text1.len();
    let M = text2.len();
    let mut dp = vec![vec![0; M + 1]; N + 1];

    for i in 0..N {
        for j in 0..M {
            if text1.as_bytes()[i] == text2.as_bytes()[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
            }
        }
    }

    dp[N][M]
}

fn main() {
    let text1 = "abcde".to_string();
    let text2 = "ace".to_string();
    assert_eq!(longest_common_subsequence(text1, text2), 3);

    let text1 = "abc".to_string();
    let text2 = "abc".to_string();
    assert_eq!(longest_common_subsequence(text1, text2), 3);

    let text1 = "abc".to_string();
    let text2 = "def".to_string();
    assert_eq!(longest_common_subsequence(text1, text2), 0);
}

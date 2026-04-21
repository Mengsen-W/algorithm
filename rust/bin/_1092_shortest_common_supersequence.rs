/*
 * @Date: 2023-03-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-28
 * @FilePath: /algorithm/rust/1092_shortest_common_supersequence/shortest_common_supersequence.rs
 */

pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
    let s1: Vec<char> = str1.chars().collect();
    let s2: Vec<char> = str2.chars().collect();
    let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];

    for i in 0..s1.len() + 1 {
        dp[i][0] = 0;
    }

    for j in 0..s2.len() + 1 {
        dp[0][j] = 0;
    }

    for i in 0..s1.len() {
        for j in 0..s2.len() {
            if s1[i] == s2[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }

    let mut sub_s = String::new();
    let mut i = s1.len();
    let mut j = s2.len();

    while i > 0 || j > 0 {
        if i == 0 {
            sub_s.push(s2[j - 1]);
            j -= 1;
        } else if j == 0 {
            sub_s.push(s1[i - 1]);
            i -= 1;
        } else if s1[i - 1] == s2[j - 1] {
            sub_s.push(s1[i - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            sub_s.push(s1[i - 1]);
            i -= 1;
        } else {
            sub_s.push(s2[j - 1]);
            j -= 1;
        }
    }

    sub_s.chars().rev().collect()
}

fn main() {
    let str1 = "abac".to_string();
    let str2 = "cab".to_string();
    let ans = "cabac".to_string();
    assert_eq!(shortest_common_supersequence(str1, str2), ans);
}

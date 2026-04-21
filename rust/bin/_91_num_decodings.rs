/*
 * @Date: 2021-04-21 08:49:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-21 09:11:28
 */

fn num_decodings(s: String) -> i32 {
    let s = s.chars().collect::<Vec<char>>();
    if s.len() == 0 || (s.len() == 1 && s[0] == '0') {
        return 0;
    }
    if s.len() == 1 {
        return 1;
    }
    let mut dp: Vec<i32> = vec![0; s.len() + 1];
    dp[0] = 1;
    for i in 0..s.len() {
        if s[i] != '0' {
            dp[i + 1] = dp[i];
        }
        if i > 0 && (s[i - 1] == '1' || (s[i - 1] == '2' && s[i] <= '6')) {
            dp[i + 1] += dp[i - 1];
        }
    }
    return *dp.last().unwrap();
}

fn main() {
    assert_eq!(num_decodings("12".to_string()), 2);
    assert_eq!(num_decodings("226".to_string()), 3);
    assert_eq!(num_decodings("0".to_string()), 0);
    assert_eq!(num_decodings("06".to_string()), 0);
}

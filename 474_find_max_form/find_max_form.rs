/*
 * @Date: 2021-06-06 09:36:37
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-06 10:04:19
 */

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

fn main() {
    {
        let strs = vec![
            "10".to_string(),
            "0001".to_string(),
            "111001".to_string(),
            "1".to_string(),
            "0".to_string(),
        ];
        assert_eq!(find_max_form(strs, 5, 3), 4);
    }
    {
        let strs = vec!["10".to_string(), "0".to_string(), "1".to_string()];
        assert_eq!(find_max_form(strs, 1, 1), 2);
    }
}

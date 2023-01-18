/*
 * @Date: 2022-06-01 09:34:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-01 09:40:07
 * @FilePath: /algorithm/473_makesquare/makesquare.rs
 */

pub fn makesquare(matchsticks: Vec<i32>) -> bool {
    if matchsticks.len() < 4 {
        return false;
    }
    let sum = matchsticks.iter().sum::<i32>();
    if sum & 3 != 0 {
        return false;
    }
    let a = sum >> 2;
    let mut dp = vec![-1; 1 << matchsticks.len()];
    dp[0] = 0;
    for s in 1..dp.len() {
        for k in 0..matchsticks.len() {
            if s & (1 << k) == 0 {
                continue;
            }
            let s1 = s & !(1 << k);
            if dp[s1] >= 0 && dp[s1] + matchsticks[k] <= a {
                dp[s] = (dp[s1] + matchsticks[k]) % a;
                break;
            }
        }
    }
    *dp.last().unwrap() == 0
}

fn main() {
    assert_eq!(makesquare(vec![1, 1, 2, 2, 2]), true);
    assert_eq!(makesquare(vec![3, 3, 3, 3, 4]), false);
}

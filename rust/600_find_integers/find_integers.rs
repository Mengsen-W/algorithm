/*
 * @Date: 2021-09-11 07:53:05
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-11 08:07:39
 */

struct Solution;

impl Solution {
    pub fn find_integers(mut n: i32) -> i32 {
        let mut dp: Vec<i32> = vec![0; 31];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..31 {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        let (mut pre, mut res) = (0, 0);
        for i in (0..=29).rev() {
            let val = 1 << i;
            if (n & val) != 0 {
                n -= val;
                res += dp[i + 1];
                if pre == 1 {
                    break;
                }
                pre = 1;
            } else {
                pre = 0;
            }
            if i == 0 {
                res += 1;
            }
        }
        res
    }
}

fn main() {
    assert_eq!(Solution::find_integers(5), 5);
}

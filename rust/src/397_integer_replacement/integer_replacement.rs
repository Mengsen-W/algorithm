/*
 * @Date: 2021-11-19 00:29:33
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-19 00:48:30
 */

struct Solution;

impl Solution {
    pub fn integer_replacement(mut n: i32) -> i32 {
        let mut ans = 0;
        while n != 1 {
            if n % 2 == 0 {
                ans += 1;
                n /= 2;
            } else if n % 4 == 1 {
                ans += 2;
                n /= 2;
            } else {
                if n == 3 {
                    ans += 2;
                } else {
                    ans += 2;
                    n = n / 2 + 1;
                }
            }
        }
        ans
    }
}

fn main() {
    assert_eq!(Solution::integer_replacement(8), 3);
    assert_eq!(Solution::integer_replacement(7), 4);
    assert_eq!(Solution::integer_replacement(4), 2);
}

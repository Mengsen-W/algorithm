/*
 * @Date: 2021-09-27 09:32:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-27 11:10:07
 */

struct Solution;

impl Solution {
    const MOD: i64 = 1000000007;
    pub fn num_decodings(s: String) -> i32 {
        let check1digit = |ch: char| -> i64 {
            match ch {
                '0' => 0,
                '*' => 9,
                _ => 1,
            }
        };
        let check2digit = |c0: char, c1: char| -> i64 {
            match (c0, c1) {
                ('*', '*') => 15,
                ('*', _) if c1 <= '6' => 2,
                ('*', _) if c1 > '6' => 1,
                ('1', '*') => 9,
                ('2', '*') => 6,
                (_, '*') => 0,
                (_, _) => {
                    (c0 != '0' && ((c0 as u32 - '0' as u32) * 10 + (c1 as u32 - '0' as u32) <= 26))
                        as i64
                }
            }
        };

        let n = s.len();
        let (mut a, mut b, mut c) = (0, 1, 0);
        for i in 1..=n {
            c = b * check1digit(s.chars().nth(i - 1).unwrap()) % Self::MOD;
            if i > 1 {
                c = (c + a * check2digit(
                    s.chars().nth(i - 2).unwrap(),
                    s.chars().nth(i - 1).unwrap(),
                )) % Self::MOD;
            }
            a = b;
            b = c
        }
        c as i32
    }
}

fn main() {
    assert_eq!(Solution::num_decodings("*".to_string()), 9);
    assert_eq!(Solution::num_decodings("1*".to_string()), 18);
    assert_eq!(Solution::num_decodings("2*".to_string()), 15);
    assert_eq!(
        Solution::num_decodings("7*9*3*6*3*0*5*4*9*7*3*7*1*8*3*2*0*0*6*".to_string()),
        196465252
    );
}

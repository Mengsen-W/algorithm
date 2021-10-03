/*
 * @Date: 2021-10-03 09:05:53
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-03 09:09:35
 */

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0 {
            return "0".to_string();
        }
        let mut s = String::new();
        if (numerator < 0) ^ (denominator < 0) {
            s.push('-');
        }
        let (numerator, denominator) = ((numerator as i64).abs(), (denominator as i64).abs());
        let (integer, mut remainder) = (numerator / denominator, numerator % denominator);
        s.push_str(&integer.to_string());
        if remainder != 0 {
            s.push('.');
        }
        let mut map = HashMap::new();
        while remainder != 0 && !map.contains_key(&remainder) {
            map.insert(remainder, s.len());
            remainder *= 10;
            s.push_str(&(remainder / denominator).to_string());
            remainder %= denominator;
        }
        if let Some(pos) = map.get(&remainder) {
            s.insert(*pos, '(');
            s.push(')');
        }
        s
    }
}

fn main() {
    assert_eq!(Solution::fraction_to_decimal(1, 2), "0.5");
    assert_eq!(Solution::fraction_to_decimal(2, 1), "2");
    assert_eq!(Solution::fraction_to_decimal(2, 3), "0.(6)");
    assert_eq!(Solution::fraction_to_decimal(4, 333), "0.(012)");
    assert_eq!(Solution::fraction_to_decimal(1, 5), "0.2");
}

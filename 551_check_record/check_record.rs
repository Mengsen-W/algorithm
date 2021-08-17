/*
 * @Date: 2021-08-17 09:24:05
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-17 09:32:53
 */

struct Solution;

impl Solution {
    pub fn check_record(s: String) -> bool {
        !(s.find("LLL").is_some() || s.find('A') != s.rfind('A'))
    }
}

fn main() {
    assert!(Solution::check_record("PPALLP".to_string()));
    assert!(!Solution::check_record("PPALLL".to_string()));
}

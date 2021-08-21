/*
 * @Date: 2021-08-20 15:16:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-21 14:12:03
 */

struct Solution;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let k = k as usize;
        let mut res = s.clone();
        let mut left: usize = 0;
        let mut right: usize;
        for _ in (0..s.len()).step_by(2 * k) {
            right = s.len().min(left + k);
            let word = s[left..right].chars().rev().collect::<String>();
            res.replace_range(left..right, word.as_str());
            left += 2 * k;
        }
        res
    }
}

fn main() {
    {
        let s = "abcdefg".to_string();
        let k = 2;
        let ans = "bacdfeg";
        assert_eq!(Solution::reverse_str(s, k), ans);
    }
    {
        let s = "abcd".to_string();
        let k = 2;
        let ans = "bacd";
        assert_eq!(Solution::reverse_str(s, k), ans);
    }
}

/*
 * @Date: 2021-09-12 08:20:00
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-12 08:39:07
 */

struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let (mut min_count, mut max_count) = (0, 0);
        let n = s.len();
        for i in 0..n {
            let c = s[i];
            if c == '(' {
                min_count += 1;
                max_count += 1;
            } else if c == ')' {
                min_count = (min_count - 1).max(0);
                max_count -= 1;
                if max_count < 0 {
                    return false;
                }
            } else {
                min_count = (min_count - 1).max(0);
                max_count += 1;
            }
        }
        min_count == 0
    }
}

fn main() {
    {
        let s = "()".to_string();
        assert!(Solution::check_valid_string(s));
    }
    {
        let s = "(*)".to_string();
        assert!(Solution::check_valid_string(s));
    }
    {
        let s = "(*))".to_string();
        assert!(Solution::check_valid_string(s));
    }
}

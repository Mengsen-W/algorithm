/*
 * @Date: 2024-01-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-10
 * @FilePath: /algorithm/rust/2696_min_length/min_length.rs
 */

struct Solution;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        use std::collections::VecDeque;
        let mut stack: VecDeque<char> = VecDeque::new();
        for c in s.chars() {
            if c == 'D' && !stack.is_empty() && *(stack.back().unwrap()) == 'C' {
                stack.pop_back();
            } else if c == 'B' && !stack.is_empty() && *(stack.back().unwrap()) == 'A' {
                stack.pop_back();
            } else {
                stack.push_back(c);
            }
        }
        stack.len() as i32
    }
}

fn main() {
    let tests = vec![("ABFCACDB", 2), ("ACBBD", 5)];

    for (s, ans) in tests {
        assert_eq!(Solution::min_length(s.to_string()), ans);
    }
}

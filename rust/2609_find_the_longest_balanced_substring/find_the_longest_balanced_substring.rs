/*
 * @Date: 2023-11-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-08
 * @FilePath: /algorithm/rust/2609_find_the_longest_balanced_substring/find_the_longest_balanced_substring.rs
 */

struct Solution;

impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let mut zero = 0;
        let mut one = 0;
        let mut ans = 0;

        for &c in s.as_bytes().iter() {
            if c == b'0' {
                if one > 0 {
                    zero = 0;
                    one = 0;
                }
                zero += 1;
            } else {
                one += 1;
                ans = std::cmp::max(ans, std::cmp::min(zero, one) * 2)
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![("01000111", 6), ("00111", 4), ("111", 0)];
    for (s, ans) in tests {
        assert_eq!(
            Solution::find_the_longest_balanced_substring(s.to_string()),
            ans
        );
    }
}

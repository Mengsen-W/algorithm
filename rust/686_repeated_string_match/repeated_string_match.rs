/*
 * @Date: 2021-12-22 00:44:24
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-22 01:54:01
 */

struct Solution;

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let (an, bn) = (a.len() as i32, b.len() as i32);
        let index = Self::str_str(&a, &b);
        if index == -1 {
            return -1;
        }
        if an - index >= bn {
            return 1;
        }
        (bn + index - an - 1) / an + 2
    }
    fn str_str(haystack: &str, needle: &str) -> i32 {
        let (n, m) = (haystack.len(), needle.len());
        if m == 0 {
            return 0;
        }
        let mut pi = vec![0; m];
        let (mut i, mut j) = (1, 0);
        while i < m {
            while j > 0 && needle.chars().nth(i) != needle.chars().nth(j) {
                j = pi[j as usize - 1];
            }
            if needle.chars().nth(i) == needle.chars().nth(j) {
                j += 1;
            }
            pi[i as usize] = j;
            i += 1;
        }

        let (mut i, mut j) = (0, 0);
        while i - j < n {
            while j > 0 && haystack.chars().nth(i % n) != needle.chars().nth(j) {
                j = pi[j - 1];
            }
            if haystack.chars().nth(i % n) == needle.chars().nth(j) {
                j += 1;
            }
            if j == m {
                return i as i32 - m as i32 + 1;
            }
            i += 1;
        }
        -1
    }
}

fn main() {
    assert_eq!(
        Solution::repeated_string_match("abcd".to_string(), "cdabcdab".to_string()),
        3
    );
    assert_eq!(
        Solution::repeated_string_match("a".to_string(), "aa".to_string()),
        2
    );
    assert_eq!(
        Solution::repeated_string_match("a".to_string(), "a".to_string()),
        1
    );
    assert_eq!(
        Solution::repeated_string_match("abc".to_string(), "wxyz".to_string()),
        -1
    );
}

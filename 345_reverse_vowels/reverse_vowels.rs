/*
 * @Date: 2021-08-19 09:44:42
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-19 12:59:10
 */

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut cv: Vec<char> = s.chars().collect();
        let hs: HashSet<_> = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']
            .iter()
            .cloned()
            .collect();
        let mut p = 0;
        let mut q = cv.len() - 1;
        while p < cv.len() && p < q {
            if !hs.contains(&cv[p]) {
                p += 1;
            } else if !hs.contains(&cv[q]) {
                q -= 1;
            } else {
                cv.swap(p, q);
                p += 1;
                q -= 1;
            }
        }
        cv.into_iter().collect()
    }
}

fn main() {
    {
        let s = "hello".to_string();
        let ans = "holle".to_string();
        assert_eq!(Solution::reverse_vowels(s), ans)
    }
    {
        let s = "leetcode".to_string();
        let ans = "leotcede".to_string();
        assert_eq!(Solution::reverse_vowels(s), ans)
    }
}

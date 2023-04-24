/*
 * @Date: 2023-04-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-24
 * @FilePath: /algorithm/rust/1063_last_substring/last_substring.rs
 */

pub fn last_substring(s: String) -> String {
    let mut i = 0;
    let mut j = i + 1;
    let mut k = 0;
    while i + k < s.len() && j + k < s.len() {
        if s.as_bytes()[i + k] == s.as_bytes()[j + k] {
            k += 1;
        } else if s.as_bytes()[i + k] < s.as_bytes()[j + k] {
            i = i + k + 1;
            k = 0;
            if i >= j {
                j = i + 1;
            }
        } else {
            j = j + k + 1;
            k = 0;
        }
    }
    return s[i..].to_string();
}

fn main() {
    {
        let s = "abab".to_string();
        let ans = "bab".to_string();
        assert_eq!(last_substring(s), ans);
    }

    {
        let s = "leetcode".to_string();
        let ans = "tcode".to_string();
        assert_eq!(last_substring(s), ans);
    }
}

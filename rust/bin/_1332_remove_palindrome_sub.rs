/*
 * @Date: 2022-01-22 09:14:01
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-22 09:16:57
 */

pub fn remove_palindrome_sub(s: String) -> i32 {
    if s.is_empty() {
        0
    } else if s == s.chars().rev().collect::<String>() {
        1
    } else {
        2
    }
}

fn main() {
    assert_eq!(remove_palindrome_sub("ababa".to_string()), 1);
    assert_eq!(remove_palindrome_sub("abb".to_string()), 2);
    assert_eq!(remove_palindrome_sub("babb".to_string()), 2);
}

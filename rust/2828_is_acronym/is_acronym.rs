/*
 * @Date: 2023-12-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-20
 * @FilePath: /algorithm/rust/2828_is_acronym/is_acronym.rs
 */

struct Solution;

impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        if words.len() != s.len() {
            return false;
        }
        for (i, w) in words.iter().enumerate() {
            if w.chars().next().unwrap_or_default() != s.chars().nth(i).unwrap_or_default() {
                return false;
            }
        }
        true
    }
}

fn main() {
    let tests = vec![
        (vec!["alice", "bob", "charlie"], "abc", true),
        (vec!["an", "apple"], "a", false),
        (
            vec!["never", "gonna", "give", "up", "on", "you"],
            "ngguoy",
            true,
        ),
    ];

    for (words, s, ans) in tests {
        assert_eq!(
            Solution::is_acronym(words.iter().map(|s| s.to_string()).collect(), s.to_string()),
            ans
        );
    }
}

/*
 * @Date: 2024-01-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-17
 * @FilePath: /algorithm/rust/2744_maximum_number_of_string_pairs/maximum_number_of_string_pairs.rs
 */

struct Solution;

impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        use std::collections::HashSet;
        let mut seen: HashSet<u16> = HashSet::new();
        let mut ans = 0;
        for word in words {
            let key =
                word.chars().nth(1).unwrap() as u16 * 100 + word.chars().nth(0).unwrap() as u16;
            if seen.contains(&key) {
                ans += 1;
            }
            seen.insert(
                word.chars().nth(0).unwrap() as u16 * 100 + word.chars().nth(1).unwrap() as u16,
            );
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec!["cd", "ac", "dc", "ca", "zz"], 2),
        (vec!["ab", "ba", "cc"], 1),
        (vec!["aa", "ab"], 0),
    ];

    for (words, ans) in tests {
        assert_eq!(
            Solution::maximum_number_of_string_pairs(words.iter().map(|s| s.to_string()).collect()),
            ans
        );
    }
}

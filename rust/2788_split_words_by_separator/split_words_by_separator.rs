/*
 * @Date: 2024-01-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-21
 * @FilePath: /algorithm/rust/2788_split_words_by_separator/split_words_by_separator.rs
 */

struct Solution;

impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        words
            .iter()
            .flat_map(|s| s.split(separator))
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
    }
}

fn main() {
    let tests = vec![
        (
            vec!["one.two.three", "four.five", "six"],
            '.',
            vec!["one", "two", "three", "four", "five", "six"],
        ),
        (vec!["$easy$", "$problem$"], '$', vec!["easy", "problem"]),
        (vec!["|||"], '|', vec![]),
    ];

    for (words, separator, ans) in tests {
        assert_eq!(
            Solution::split_words_by_separator(
                words.iter().map(|s| s.to_string()).collect(),
                separator
            ),
            ans
        );
    }
}

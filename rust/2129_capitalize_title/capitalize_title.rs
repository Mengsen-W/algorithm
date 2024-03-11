/*
 * @Date: 2024-03-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-11
 * @FilePath: /algorithm/rust/2129_capitalize_title/capitalize_title.rs
 */

struct Solution;

impl Solution {
    pub fn capitalize_title(title: String) -> String {
        title
            .split(" ")
            .map(|s| {
                if s.len() > 2 {
                    s[..1].to_uppercase() + &s[1..].to_lowercase()
                } else {
                    s.to_lowercase()
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn main() {
    let tests = vec![
        ("capiTalIze tHe titLe", "Capitalize The Title"),
        ("First leTTeR of EACH Word", "First Letter of Each Word"),
        ("i lOve leetcode", "i Love Leetcode"),
    ];

    for (title, ans) in tests {
        assert_eq!(Solution::capitalize_title(title.to_string()), ans);
    }
}

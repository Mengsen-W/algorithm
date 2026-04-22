/*
 * @Date: 2024-01-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-09
 * @FilePath: /algorithm/rust/2707_min_extra_char/min_extra_char.rs
 */

struct Solution;

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        use std::cmp;
        use std::collections::HashSet;
        let n = s.len();
        let mut d = vec![std::i32::MAX; n + 1];
        d[0] = 0;
        let mut mp = HashSet::new();
        for e in dictionary {
            mp.insert(e);
        }
        for i in 1..=n {
            d[i] = d[i - 1] + 1;
            for j in (0..i).rev() {
                if mp.contains(&s[j..i]) {
                    d[i] = cmp::min(d[i], d[j]);
                }
            }
        }
        d[n]
    }
}

fn main() {
    let tests = vec![
        ("leetscode", vec!["leet", "code", "leetcode"], 1),
        ("sayhelloworld", vec!["hello", "world"], 3),
    ];

    for (s, dictionary, ans) in tests {
        assert_eq!(
            Solution::min_extra_char(
                s.to_string(),
                dictionary.iter().map(|s| s.to_string()).collect()
            ),
            ans
        );
    }
}

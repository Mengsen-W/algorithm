/*
 * @Date: 2024-01-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-12
 * @FilePath: /algorithm/rust/2085_count_words/count_words.rs
 */

struct Solution;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        use std::collections::HashMap;
        let mut cnt1 = HashMap::new();
        let mut cnt2 = HashMap::new();
        words1
            .into_iter()
            .for_each(|x| *cnt1.entry(x).or_insert(0) += 1);
        words2
            .into_iter()
            .for_each(|x| *cnt2.entry(x).or_insert(0) += 1);

        let mut ans = 0;
        for (k, v) in cnt1 {
            if let Some(&u) = cnt2.get(&k) {
                if v == 1 && u == 1 {
                    ans += 1;
                }
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec!["leetcode", "is", "amazing", "as", "is"],
            vec!["amazing", "leetcode", "is"],
            2,
        ),
        (vec!["b", "bb", "bbb"], vec!["a", "aa", "aaa"], 0),
        (vec!["a", "ab"], vec!["a", "a", "a", "ab"], 1),
    ];

    for (words1, words2, ans) in tests {
        assert_eq!(
            Solution::count_words(
                words1.iter().map(|s| s.to_string()).collect(),
                words2.iter().map(|s| s.to_string()).collect()
            ),
            ans
        );
    }
}

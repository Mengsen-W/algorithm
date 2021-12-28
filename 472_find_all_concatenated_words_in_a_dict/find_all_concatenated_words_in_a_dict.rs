/*
 * @Date: 2021-12-28 00:49:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-28 02:25:34
 */

struct Solution;

impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let mut ret = Vec::new();

        words.iter().for_each(|word| {
            set.insert(word);
        });
        for word in words.iter() {
            if "" == word {
                continue;
            }
            set.remove(&word);

            if can_break(&word, &set) {
                ret.push(word.clone());
            }

            set.insert(&word);
        }

        ret
    }
}

fn can_break(s: &String, set: &HashSet<&String>) -> bool {
    let n = s.len();
    if set.is_empty() || n == 0 {
        return false;
    }

    let mut dp = vec![false; n + 1];
    dp[0] = true;

    for i in 1..n + 1 {
        for j in 0..i {
            if !dp[j] {
                continue;
            }
            if set.contains(&s[j..i].to_string()) {
                dp[i] = true;
                break;
            }
        }
    }
    dp[n]
}

fn main() {
    assert_eq!(
        Solution::find_all_concatenated_words_in_a_dict(
            vec![
                "cat",
                "cats",
                "catsdogcats",
                "dog",
                "dogcatsdog",
                "hippopotamuses",
                "rat",
                "ratcatdogcat"
            ]
            .iter()
            .map(|i| i.to_string())
            .collect()
        ),
        vec!["catsdogcats", "dogcatsdog", "ratcatdogcat"]
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
    );
    assert_eq!(
        Solution::find_all_concatenated_words_in_a_dict(
            vec!["cat", "dog", "catdog"]
                .iter()
                .map(|i| i.to_string())
                .collect()
        ),
        vec!["catdog"]
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
    );
}

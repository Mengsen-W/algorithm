struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let mut words_perfect = HashSet::new();
        let mut words_cap = HashMap::new();
        let mut words_vow = HashMap::new();

        fn devowel(word: &str) -> String {
            word.chars()
                .map(|c| if is_vowel(c) { '*' } else { c })
                .collect()
        }

        fn is_vowel(c: char) -> bool {
            match c.to_ascii_lowercase() {
                'a' | 'e' | 'i' | 'o' | 'u' => true,
                _ => false,
            }
        }

        for word in &wordlist {
            words_perfect.insert(word.clone());
            let wordlow = word.to_ascii_lowercase();
            words_cap.entry(wordlow.clone()).or_insert(word.clone());
            let wordlow_dv = devowel(&wordlow);
            words_vow.entry(wordlow_dv).or_insert(word.clone());
        }

        let mut res = Vec::with_capacity(queries.len());
        for query in queries {
            if words_perfect.contains(&query) {
                res.push(query);
                continue;
            }
            let query_l = query.to_ascii_lowercase();
            if let Some(word) = words_cap.get(&query_l) {
                res.push(word.clone());
                continue;
            }
            let query_lv = devowel(&query_l);
            if let Some(word) = words_vow.get(&query_lv) {
                res.push(word.clone());
                continue;
            }
            res.push(String::new());
        }
        res
    }
}

fn main() {
    let tests = vec![
        (
            vec!["KiTe", "kite", "hare", "Hare"],
            vec![
                "kite", "Kite", "KiTe", "Hare", "HARE", "Hear", "hear", "keti", "keet", "keto",
            ],
            vec![
                "kite", "KiTe", "KiTe", "Hare", "hare", "", "", "KiTe", "", "KiTe",
            ],
        ),
        (vec!["yellow"], vec!["YellOw"], vec!["yellow"]),
    ];

    for (wordlist, queries, ans) in tests {
        assert_eq!(
            Solution::spellchecker(
                wordlist.into_iter().map(|s| s.to_string()).collect(),
                queries.into_iter().map(|s| s.to_string()).collect()
            ),
            ans
        );
    }
}

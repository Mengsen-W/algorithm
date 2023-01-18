/*
 * @Date: 2022-01-30 00:59:38
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-30 01:23:05
 */

pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    use std::collections::HashMap;
    let mut tab = HashMap::new();
    s1.split_ascii_whitespace()
        .chain(s2.split_ascii_whitespace())
        .for_each(|s| *tab.entry(s).or_insert(0) += 1);
    tab.into_iter()
        .filter(|(_, v)| *v == 1)
        .map(|(k, _)| k.to_string())
        .collect()
}

fn main() {
    assert_eq!(
        uncommon_from_sentences(
            "this apple is sweet".to_string(),
            "this apple is sour".to_string()
        ),
        vec!["sour", "sweet"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );

    assert_eq!(
        uncommon_from_sentences("apple apple".to_string(), "banana".to_string()),
        vec!["banana".to_string()]
    );
}

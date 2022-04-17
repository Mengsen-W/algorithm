/*
 * @Date: 2022-04-17 09:05:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-17 09:30:59
 * @FilePath: /algorithm/819_most_common_word/most_common_word.rs
 */

pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
    use std::collections::{HashMap, HashSet};
    paragraph
        .to_ascii_lowercase()
        .split(&[' ', '!', '?', '\'', ',', ';', '.'][..])
        .fold(
            (
                HashMap::new(),
                banned.into_iter().collect::<HashSet<String>>(),
            ),
            |(mut mp, ban), s| {
                if !ban.contains(s) && !s.is_empty() {
                    mp.entry(s).and_modify(|e| *e += 1).or_insert(1);
                }
                (mp, ban)
            },
        )
        .0
        .into_iter()
        .max_by_key(|e| e.1)
        .unwrap()
        .0
        .to_string()
}

fn main() {
    assert_eq!(
        most_common_word(
            String::from("Bob hit a ball, the hit BALL flew far after it was hit."),
            vec![String::from("hit")]
        ),
        String::from("ball")
    );
}

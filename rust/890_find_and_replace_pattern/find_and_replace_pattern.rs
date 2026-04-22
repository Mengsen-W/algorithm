/*
 * @Date: 2022-06-12 11:04:01
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-12 11:19:20
 * @FilePath: /algorithm/890_find_and_replace_pattern/find_and_replace_pattern.rs
 */

pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
    fn is_match(word: &String, pattern: &String) -> bool {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for i in 0..word.len() {
            let x = word.chars().nth(i).unwrap();
            let y = pattern.chars().nth(i).unwrap();
            if !map.contains_key(&x) {
                map.insert(x, y);
            } else if map.get(&x).unwrap() != &y {
                return false;
            }
        }
        true
    }
    words
        .into_iter()
        .filter(|word| is_match(&word, &pattern) && is_match(&pattern, &word))
        .collect()
}

fn main() {
    let words = vec!["abc", "deq", "mee", "aqq", "dkd", "ccc"]
        .iter()
        .map(|x| x.to_string())
        .collect();
    let pattern = String::from("abb");
    let ans = vec!["mee", "aqq"];
    assert_eq!(find_and_replace_pattern(words, pattern), ans)
}

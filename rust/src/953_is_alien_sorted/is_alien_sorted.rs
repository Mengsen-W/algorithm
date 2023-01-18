/*
 * @Date: 2022-05-17 09:33:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-17 09:40:39
 * @FilePath: /algorithm/953_is_alien_sorted/is_alien_sorted.rs
 */
pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
    let freq = order
        .as_bytes()
        .iter()
        .enumerate()
        .fold(vec![0; 26], |mut freq, (i, b)| {
            freq[(b - b'a') as usize] = i;
            freq
        });
    words
        .iter()
        .map(|s| {
            s.as_bytes()
                .iter()
                .map(|&b| freq[(b - b'a') as usize])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .windows(2)
        .all(|words| words[0] <= words[1])
}

fn main() {
    assert_eq!(
        is_alien_sorted(
            vec!["hello".to_string(), "leetcode".to_string()],
            "hlabcdefgijkmnopqrstuvwxyz".to_string()
        ),
        true
    );

    assert_eq!(
        is_alien_sorted(
            vec!["word".to_string(), "world".to_string(), "row".to_string()],
            "worldabcefghijkmnpqstuvxyz".to_string()
        ),
        false
    );

    assert_eq!(
        is_alien_sorted(
            vec!["apple".to_string(), "app".to_string()],
            "abcdefghijklmnopqrstuvwxyz".to_string()
        ),
        false
    );
}

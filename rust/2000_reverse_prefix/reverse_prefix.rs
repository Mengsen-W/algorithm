/*
 * @Date: 2022-02-02 00:53:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-02 01:02:47
 */

pub fn reverse_prefix(word: String, ch: char) -> String {
    match word.find(ch) {
        Some(i) => word[0..=i].chars().rev().collect::<String>() + &word[i + 1..],
        _ => word,
    }
}

fn main() {
    assert_eq!(
        reverse_prefix("abcdefd".to_string(), 'd'),
        "dcbaefd".to_string()
    );
    assert_eq!(
        reverse_prefix("xyxzxe".to_string(), 'z'),
        "zxyxxe".to_string()
    );
    assert_eq!(reverse_prefix("abcd".to_string(), 'z'), "abcd".to_string());
}

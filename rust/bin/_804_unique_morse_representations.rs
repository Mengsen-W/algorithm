/*
 * @Date: 2022-04-10 09:53:40
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-10 10:37:41
 * @FilePath: /algorithm/804_unique_morse_representations/unique_morse_representations.rs
 */

pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    const MORSE: [&str; 26] = [
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..",
    ];
    let mut set = std::collections::HashSet::new();
    for word in words {
        let mut code = String::new();
        for c in word.chars() {
            code.push_str(MORSE[(c as u8 - 'a' as u8) as usize]);
        }
        set.insert(code);
    }
    set.len() as i32
}

fn main() {
    assert_eq!(
        unique_morse_representations(
            vec!["gin", "zen", "gig", "msg"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        ),
        2
    );
    assert_eq!(
        unique_morse_representations(vec!["a"].into_iter().map(|s| s.to_string()).collect()),
        1
    );
}

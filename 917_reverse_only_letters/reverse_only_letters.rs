/*
 * @Date: 2022-02-23 12:53:51
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-23 13:00:25
 * @FilePath: /algorithm/917_reverse_only_letters/reverse_only_letters.rs
 */

pub fn reverse_only_letters(s: String) -> String {
    let mut chars = s.chars().collect::<Vec<char>>();
    let (mut start, mut end) = (0, chars.len() - 1);
    while start < end {
        while start < end && !chars[start].is_alphabetic() {
            start += 1;
        }
        while start < end && !chars[end].is_alphabetic() {
            end -= 1;
        }
        if start < end {
            chars.swap(start, end);
        }
        start += 1;
        end -= 1;
    }
    chars.iter().collect::<String>()
}

fn main() {
    assert_eq!(
        reverse_only_letters("ab-cd".to_string()),
        "dc-ba".to_string()
    );
    assert_eq!(
        reverse_only_letters("a-bC-dEf-ghIj".to_string()),
        "j-Ih-gfE-dCba".to_string()
    );
    assert_eq!(
        reverse_only_letters("Test1ng-Leet=code-Q!".to_string()),
        "Qedo1ct-eeLg=ntse-T!".to_string()
    );
}

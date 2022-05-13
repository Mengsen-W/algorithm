/*
 * @Date: 2022-05-13 09:27:54
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-13 09:37:13
 * @FilePath: /algorithm/01.05_one_edit_away/one_edit_away.rs
 */

pub fn one_edit_away(first: String, second: String) -> bool {
    if first == second {
        return true;
    }

    let first_chars: Vec<char> = first.chars().collect();
    let second_chars: Vec<char> = second.chars().collect();
    let first_len = first_chars.len();
    let second_len = second_chars.len();

    if (first_len as i32 - second_len as i32).abs() > 1 {
        return false;
    }

    let mut i = 0;
    let mut j = 0;

    while i < first_len && j < second_len {
        if first_chars[i] == second_chars[j] {
            i += 1;
            j += 1;
            continue;
        }

        if first_len == second_len {
            return first_chars[i + 1..] == second_chars[i + 1..];
        } else if first_len > second_len {
            return first_chars[i + 1..] == second_chars[i..];
        }
        return &first_chars[i..] == &second_chars[i + 1..];
    }
    true
}

fn main() {
    assert_eq!(one_edit_away("pale".to_string(), "ple".to_string()), true);
    assert_eq!(one_edit_away("pales".to_string(), "pla".to_string()), true);
}

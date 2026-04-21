/*
 * @Date: 2021-12-12 05:21:54
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-12 05:38:08
 * @FilePath: /algorithm/709_to_lower_case/to_lower_case.rs
 * @Description: file content
 */

pub fn to_lower_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c >= 'A' && c <= 'Z' {
                (c as u8 + 32 as u8) as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    assert_eq!(to_lower_case("Hello".to_string()), "hello".to_string());
    assert_eq!(to_lower_case("here".to_string()), "here".to_string());
    assert_eq!(to_lower_case("LOVELY".to_string()), "lovely".to_string());
}

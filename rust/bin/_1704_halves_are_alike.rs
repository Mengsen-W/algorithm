/*
 * @Date: 2022-11-11
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-11
 * @FilePath: /algorithm/1704_halves_are_alike/halves_are_alike.rs
 */

pub fn halves_are_alike(s: String) -> bool {
    s.bytes()
        .enumerate()
        .filter(|(_, c)| b"aeiouAEIOU".contains(c))
        .map(|(i, _)| if i < s.len() / 2 { 1 } else { -1 })
        .sum::<i32>()
        == 0
}

fn main() {
    assert!(halves_are_alike(String::from("book")));
    assert!(!halves_are_alike(String::from("textbook")));
}

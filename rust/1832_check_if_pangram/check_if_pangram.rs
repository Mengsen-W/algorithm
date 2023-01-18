/*
 * @Date: 2022-12-13
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-13
 * @FilePath: /algorithm/1832_check_if_pangram/check_if_pangram.rs
 */

pub fn check_if_pangram(sentence: String) -> bool {
    sentence
        .bytes()
        .fold(-(1 << 26), |acc, u| acc | (1 << (u - b'a')))
        == -1
}

fn main() {
    assert!(check_if_pangram(String::from(
        "thequickbrownfoxjumpsoverthelazydog"
    )));
    assert!(!check_if_pangram(String::from("leetcode")));
}

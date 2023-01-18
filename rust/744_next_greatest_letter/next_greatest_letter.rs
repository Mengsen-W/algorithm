/*
 * @Date: 2022-04-02 23:34:05
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-02 23:45:24
 * @FilePath: /algorithm/744_next_greatest_letter/next_greatest_letter.rs
 */

pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    *letters.iter().find(|&c| c > &target).unwrap_or(&letters[0])
}

fn main() {
    assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'a'), 'c');
    assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'c'), 'f');
    assert_eq!(next_greatest_letter(vec!['c', 'f', 'd'], 'c'), 'f');
}

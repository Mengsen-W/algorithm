/*
 * @Date: 2023-01-01
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2023-01-01
 * @FilePath: /algorithm/2351_repeated_character/repeated_character.rs
 */

pub fn repeated_character(s: String) -> char {
    let mut a = 0;
    for c in s.chars() {
        if a & (1 << (c as u8 - b'a')) > 0 {
            return c;
        } else {
            a |= 1 << (c as u8 - b'a')
        }
    }
    '\0'
}

fn main() {
    {
        let s = String::from("abccbaacz");
        let ans = 'c';
        assert_eq!(repeated_character(s), ans);
    }

    {
        let s = String::from("abcdd");
        let ans = 'd';
        assert_eq!(repeated_character(s), ans);
    }
}

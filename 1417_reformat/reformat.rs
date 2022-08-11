/*
 * @Date: 2022-08-11
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-11
 * @FilePath: /algorithm/1417_reformat/reformat.rs
 */

pub fn reformat(s: String) -> String {
    use std::mem::swap;
    let (mut letter, mut digit, mut ret) = (Vec::new(), Vec::new(), Vec::new());
    for ch in s.chars() {
        if ch.is_ascii_digit() {
            digit.push(ch);
        } else {
            letter.push(ch);
        }
    }
    if (letter.len() as i32 - digit.len() as i32).abs() > 1 {
        return String::new();
    }
    if letter.len() < digit.len() {
        swap(&mut letter, &mut digit);
    }
    for i in 0..digit.len() {
        ret.push(letter[i]);
        ret.push(digit[i]);
    }
    if letter.len() != digit.len() {
        ret.push(letter.last().copied().unwrap());
    }
    ret.iter().collect::<String>()
}

fn main() {
    assert_eq!(reformat(String::from("a0b1c2")), String::from("a0b1c2"));
    assert_eq!(reformat(String::from("leetcode")), String::from(""));
    assert_eq!(reformat(String::from("1229857369")), String::from(""));
    assert_eq!(
        reformat(String::from("covid2019")),
        String::from("c2o0v1i9d")
    );
    assert_eq!(reformat(String::from("ab123")), String::from("1a2b3"));
}

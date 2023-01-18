/*
 * @Date: 2022-03-07 00:02:58
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-07 00:19:04
 * @FilePath: /algorithm/504_convert_to_base7/convert_to_base7.rs
 */

pub fn convert_to_base7(mut num: i32) -> String {
    if num == 0 {
        return String::from("0");
    }
    let is_neg = num < 0;
    if is_neg {
        num = num.abs();
    }
    let mut ans = String::new();
    while num > 0 {
        ans.push(char::from_digit((num % 7) as u32, 7).unwrap());
        num /= 7;
    }
    if is_neg {
        ans.push('-');
    }
    ans.chars().rev().collect()
}

fn main() {
    assert_eq!(convert_to_base7(100), "202".to_string());
    assert_eq!(convert_to_base7(-7), "-10".to_string());
}

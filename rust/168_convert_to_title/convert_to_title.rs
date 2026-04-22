/*
 * @Date: 2021-06-29 08:49:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-29 09:02:44
 */

fn convert_to_title(mut column_number: i32) -> String {
    let mut answer = String::new();
    while column_number > 0 {
        column_number -= 1;
        answer.push(((column_number % 26) as u8 + 'A' as u8) as char);
        column_number /= 26;
    }
    answer.chars().rev().collect::<String>()
}

fn main() {
    assert_eq!(convert_to_title(1), "A".to_string());
    assert_eq!(convert_to_title(28), "AB".to_string());
    assert_eq!(convert_to_title(701), "ZY".to_string());
}

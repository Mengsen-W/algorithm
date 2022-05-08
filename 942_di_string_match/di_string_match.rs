/*
 * @Date: 2022-05-09 07:38:39
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-09 07:44:38
 * @FilePath: /algorithm/942_di_string_match/di_string_match.rs
 */

pub fn di_string_match(s: String) -> Vec<i32> {
    let (mut min, mut max) = (0, s.len() as i32);
    let mut answer = Vec::new();
    for x in s.into_bytes() {
        if x == 'I' as u8 {
            answer.push(min);
            min += 1;
        } else {
            answer.push(max);
            max -= 1;
        }
    }
    answer.push(min);
    answer
}

fn main() {
    assert_eq!(di_string_match("IDID".to_string()), vec![0, 4, 1, 3, 2]);
    assert_eq!(di_string_match("III".to_string()), vec![0, 1, 2, 3]);
    assert_eq!(di_string_match("DDI".to_string()), vec![3, 2, 0, 1]);
}

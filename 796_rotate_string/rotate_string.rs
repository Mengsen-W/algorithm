/*
 * @Date: 2022-04-07 01:43:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-07 01:50:39
 * @FilePath: /algorithm/796_rotate_string/rotate_string.rs
 */

pub fn rotate_string(s: String, goal: String) -> bool {
    s.len() == goal.len() && goal.repeat(2).contains(&s)
}

fn main() {
    assert_eq!(
        rotate_string("abcde".to_string(), "cdeab".to_string()),
        true
    );
    assert_eq!(
        rotate_string("abcde".to_string(), "abced".to_string()),
        false
    );
}

/*
 * @Date: 2022-09-17
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-17
 * @FilePath: /algorithm/1624_max_length_between_equal_characters/max_length_between_equal_characters.rs
 */

pub fn max_length_between_equal_characters(s: String) -> i32 {
    let mut pre = [i32::MAX; 26];
    let mut ans = -1;

    for (i, c) in s.chars().enumerate() {
        let c = c as usize - b'a' as usize;
        ans = ans.max(i as i32 - pre[c] - 1);
        pre[c] = pre[c].min(i as i32);
    }

    ans
}

fn main() {
    assert_eq!(max_length_between_equal_characters(String::from("aa")), 0);
    assert_eq!(max_length_between_equal_characters(String::from("abca")), 2);
    assert_eq!(
        max_length_between_equal_characters(String::from("cbzxy")),
        -1
    );
    assert_eq!(
        max_length_between_equal_characters(String::from("cabbac")),
        4
    );
}

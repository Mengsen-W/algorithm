/*
 * @Date: 2023-03-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-29
 * @FilePath: /algorithm/rust/1641_count_vowel_strings/count_vowel_strings.rs
 */

pub fn count_vowel_strings(n: i32) -> i32 {
    (n + 1) * (n + 2) * (n + 3) * (n + 4) / 24
}

fn main() {
    assert_eq!(count_vowel_strings(1), 5);
    assert_eq!(count_vowel_strings(2), 15);
}

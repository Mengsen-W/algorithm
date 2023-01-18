/*
 * @Date: 2022-09-06
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-06
 * @FilePath: /algorithm/828_unique_letter_string/unique_letter_string.rs
 */

pub fn unique_letter_string(s: String) -> i32 {
    s.char_indices()
        .fold(std::collections::HashMap::new(), |mut acc, (i, c)| {
            acc.entry(c).or_insert(vec![]).push(i as i64 + 1);
            acc
        })
        .values()
        .fold(0, |ans, values| {
            (0..values.len()).fold(ans, |mut acc, i| {
                let lo = if i > 0 { values[i - 1] } else { 0 };
                let hi = if i + 1 < values.len() {
                    values[i + 1]
                } else {
                    s.len() as i64 + 1
                };
                acc += (values[i] - lo) * (hi - values[i]);
                acc % 1_000_000_007
            })
        }) as i32
}

fn main() {
    assert_eq!(unique_letter_string(String::from("ABC")), 10);
    assert_eq!(unique_letter_string(String::from("ABA")), 8);
    assert_eq!(unique_letter_string(String::from("LEETCODE")), 92);
}

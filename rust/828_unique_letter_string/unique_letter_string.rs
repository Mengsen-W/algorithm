/*
 * @Date: 2022-09-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-26
 * @FilePath: /algorithm/rust/828_unique_letter_string/unique_letter_string.rs
 */

struct Solution;
impl Solution {
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
}

fn main() {
    let tests = vec![("ABC", 10), ("ABA", 8), ("LEETCODE", 92)];

    for (s, ans) in tests {
        assert_eq!(Solution::unique_letter_string(String::from(s)), ans);
    }
}

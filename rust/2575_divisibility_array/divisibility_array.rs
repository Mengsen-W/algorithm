/*
 * @Date: 2024-03-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-07
 * @FilePath: /algorithm/rust/2575_divisibility_array/divisibility_array.rs
 */

struct Solution;

impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let mut cur: i64 = 0;
        let mut res = Vec::new();
        for c in word.chars() {
            cur = (cur * 10 + (c.to_digit(10).unwrap() as i64)) % m as i64;
            res.push(if cur == 0 { 1 } else { 0 });
        }
        res
    }
}

fn main() {
    let tests = vec![
        ("998244353".to_string(), 3, vec![1, 1, 0, 0, 0, 1, 1, 0, 0]),
        ("1010".to_string(), 10, vec![0, 1, 0, 1]),
    ];

    for (word, m, ans) in tests {
        assert_eq!(Solution::divisibility_array(word, m), ans);
    }
}

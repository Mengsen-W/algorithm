/*
 * @Date: 2024-05-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-20
 * @FilePath: /algorithm/rust/1542_longest_awesome/longest_awesome.rs
 */

struct Solution;

impl Solution {
    pub fn longest_awesome(s: String) -> i32 {
        use std::collections::HashMap;
        let mut prefix: HashMap<i32, i32> = HashMap::new();
        prefix.insert(0, -1);
        let mut ans = 0;
        let mut sequence = 0;
        for (j, ch) in s.chars().enumerate() {
            let digit = ch.to_digit(10).unwrap() as i32;
            sequence ^= 1 << digit;
            if let Some(&prev_index) = prefix.get(&sequence) {
                ans = ans.max(j as i32 - prev_index);
            } else {
                prefix.insert(sequence, j as i32);
            }
            for k in 0..10 {
                if let Some(&prev_index) = prefix.get(&(sequence ^ (1 << k))) {
                    ans = ans.max(j as i32 - prev_index);
                }
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![("3242415", 5), ("12345678", 1), ("213123", 6)];

    for (s, ans) in tests {
        assert_eq!(Solution::longest_awesome(s.to_string()), ans);
    }
}

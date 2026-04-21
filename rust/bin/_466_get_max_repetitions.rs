/*
 * @Date: 2024-01-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-02
 * @FilePath: /algorithm/rust/466_get_max_repetitions/get_max_repetitions.rs
 */

struct Solution;

impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        use std::collections::HashMap;
        if n1 == 0 {
            return 0;
        }
        let mut s1cnt = 0;
        let mut index = 0;
        let mut s2cnt = 0;
        let mut recall = HashMap::new();
        let mut pre_loop = (0, 0);
        let mut in_loop = (0, 0);
        loop {
            s1cnt += 1;
            for ch in s1.chars() {
                if ch.to_string() == s2.chars().nth(index).unwrap().to_string() {
                    index += 1;
                    if index == s2.len() {
                        s2cnt += 1;
                        index = 0;
                    }
                }
            }
            if s1cnt == n1 {
                return s2cnt / n2;
            }
            if let Some(&(s1cnt_prime, s2cnt_prime)) = recall.get(&index) {
                pre_loop = (s1cnt_prime, s2cnt_prime);
                in_loop = (s1cnt - s1cnt_prime, s2cnt - s2cnt_prime);
                break;
            } else {
                recall.insert(index, (s1cnt, s2cnt));
            }
        }
        let mut ans = pre_loop.1 + (n1 - pre_loop.0) / in_loop.0 * in_loop.1;
        let rest = (n1 - pre_loop.0) % in_loop.0;
        for _ in 0..rest {
            for ch in s1.chars() {
                if ch.to_string() == s2.chars().nth(index).unwrap().to_string() {
                    index += 1;
                    if index == s2.len() {
                        ans += 1;
                        index = 0;
                    }
                }
            }
        }
        ans / n2
    }
}

fn main() {
    let tests = vec![("abc", 4, "ab", 2, 2), ("acb", 1, "acb", 1, 1)];

    for (s1, n1, s2, n2, ans) in tests {
        assert_eq!(
            Solution::get_max_repetitions(s1.to_string(), n1, s2.to_string(), n2),
            ans,
            "{} {} {} {}",
            s1,
            n1,
            s2,
            n2
        );
    }
}

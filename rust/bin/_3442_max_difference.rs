struct Solution;

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        use std::collections::HashMap;
        let mut c = HashMap::new();
        for ch in s.chars() {
            *c.entry(ch).or_insert(0) += 1;
        }
        let mut max_odd = 1;
        let mut min_even = s.len() as i32;
        for &value in c.values() {
            if value % 2 == 1 {
                max_odd = max_odd.max(value);
            } else {
                min_even = min_even.min(value);
            }
        }
        max_odd - min_even
    }
}

fn main() {
    let tests = vec![("aaaaabbc", 3), ("abcabcab", 1)];

    for (s, expected) in tests {
        assert_eq!(Solution::max_difference(s.to_string()), expected);
    }
}

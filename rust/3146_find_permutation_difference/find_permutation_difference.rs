struct Solution;

impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        use std::collections::HashMap;
        let mut char2index = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            char2index.insert(c, i as i32);
        }

        t.chars()
            .enumerate()
            .map(|(i, c)| (i as i32 - char2index[&c]).abs())
            .sum()
    }
}

fn main() {
    let tests = vec![("abc", "bac", 2), ("abcde", "edbac", 12)];

    for (s, t, ans) in tests {
        assert_eq!(
            Solution::find_permutation_difference(s.to_string(), t.to_string()),
            ans
        );
    }
}

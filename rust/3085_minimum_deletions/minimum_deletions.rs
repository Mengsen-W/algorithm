struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn minimum_deletions(word: String, k: i32) -> i32 {
        let mut cnt = HashMap::new();
        for c in word.chars() {
            *cnt.entry(c).or_insert(0) += 1;
        }
        let mut res = word.len() as i32;
        for &a in cnt.values() {
            let mut deleted = 0;
            for &b in cnt.values() {
                if a > b {
                    deleted += b;
                } else if b > a + k {
                    deleted += b - (a + k);
                }
            }
            res = res.min(deleted);
        }
        res
    }
}

fn main() {
    let tests = vec![("aabcaba", 0, 3), ("dabdcbdcdcd", 2, 2), ("aaabaaa", 2, 1)];

    for (word, k, ans) in tests {
        assert_eq!(Solution::minimum_deletions(word.to_string(), k), ans);
    }
}

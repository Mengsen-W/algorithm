struct Solution;

impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        use std::collections::HashMap;
        let mut res = 0;
        let mut cnt = HashMap::new();
        for word in words {
            let mut state = 0;
            for c in word.chars() {
                state |= 1 << (c as u8 - b'a');
            }
            res += cnt.get(&state).unwrap_or(&0);
            *cnt.entry(state).or_insert(0) += 1;
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec!["aba", "aabb", "abcd", "bac", "aabc"], 2),
        (vec!["aabb", "ab", "ba"], 3),
        (vec!["nba", "cba", "dba"], 0),
    ];

    for (words, ans) in tests {
        assert_eq!(
            Solution::similar_pairs(words.iter().map(|s| s.to_string()).collect()),
            ans
        );
    }
}

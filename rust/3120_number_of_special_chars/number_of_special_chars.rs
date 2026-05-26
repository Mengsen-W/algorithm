struct Solution;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        use std::collections::HashSet;
        let s: HashSet<char> = word.chars().collect();
        let mut ans = 0;
        for c in 'a'..='z' {
            if s.contains(&c) && s.contains(&c.to_uppercase().next().unwrap()) {
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![("aaAbcBC", 3), ("abc", 0), ("abBCab", 1)];

    for (word, expected) in tests {
        assert_eq!(
            Solution::number_of_special_chars(word.to_string()),
            expected
        );
    }
}

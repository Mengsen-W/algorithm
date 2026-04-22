struct Solution;

impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        let mut res = 0;
        for word in words {
            if s.starts_with(&word) {
                res += 1;
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec!["a", "b", "c", "ab", "bc", "abc"], "abc", 3),
        (vec!["a", "a"], "aa", 2),
    ];

    for (words, s, ans) in tests {
        assert_eq!(
            Solution::count_prefixes(words.iter().map(|s| s.to_string()).collect(), s.to_string()),
            ans
        );
    }
}

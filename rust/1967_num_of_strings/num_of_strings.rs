struct Solution;

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        fn check(pattern: &str, word: &str) -> bool {
            let m = pattern.len();
            let n = word.len();
            if m > n {
                return false;
            }
            let word_bytes = word.as_bytes();
            let pattern_bytes = pattern.as_bytes();

            for i in 0..=n - m {
                let mut flag = true;
                for j in 0..m {
                    if word_bytes[i + j] != pattern_bytes[j] {
                        flag = false;
                        break;
                    }
                }
                if flag {
                    return true;
                }
            }
            false
        }

        let mut res = 0;
        for pattern in patterns {
            if check(&pattern, &word) {
                res += 1;
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec!["a", "abc", "bc", "d"], "abc", 3),
        (vec!["a", "b", "c"], "aaaaabbbbb", 2),
        (vec!["a", "a", "a"], "ab", 3),
    ];

    for (patterns, word, expected) in tests {
        assert_eq!(
            Solution::num_of_strings(
                patterns.into_iter().map(|s| s.to_string()).collect(),
                word.to_string()
            ),
            expected
        );
    }
}

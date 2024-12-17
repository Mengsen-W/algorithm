struct Solution;

impl Solution {
    pub fn prefix_function(word: &str, target: &str) -> Vec<usize> {
        let s = word.to_owned() + "#" + target;
        let n = s.len();
        let mut pi = vec![0; n];
        for i in 1..n {
            let mut j = pi[i - 1];
            while j > 0 && s.as_bytes()[i] != s.as_bytes()[j] {
                j = pi[j - 1];
            }
            if s.as_bytes()[i] == s.as_bytes()[j] {
                j += 1;
            }
            pi[i] = j;
        }
        pi
    }
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let n = target.len();
        let mut back = vec![0; n];
        for word in words {
            let pi = Self::prefix_function(&word, &target);
            let m = word.len();
            for i in 0..n {
                back[i] = std::cmp::max(back[i], pi[m + 1 + i]);
            }
        }

        let mut dp = vec![0; n + 1];
        for i in 1..=n {
            dp[i] = 1_000_000_000;
        }
        for i in 0..n {
            dp[i + 1] = dp[i + 1 - back[i]] + 1;
            if dp[i + 1] > n as i32 {
                return -1;
            }
        }
        dp[n] as i32
    }
}

fn main() {
    let tests = vec![
        (vec!["abc", "aaaaa", "bcdef"], "aabcdabc", 3),
        (vec!["abababab", "ab"], "ababaababa", 2),
        (vec!["abcdef"], "xyz", -1),
    ];

    for (words, target, ans) in tests {
        assert_eq!(
            Solution::min_valid_strings(
                words.iter().map(|s| s.to_string()).collect(),
                target.to_string()
            ),
            ans
        );
    }
}

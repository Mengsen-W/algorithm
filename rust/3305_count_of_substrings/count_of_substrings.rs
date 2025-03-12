struct Solution;

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        use std::collections::{HashMap, HashSet};
        let vowels: HashSet<char> = ['a', 'e', 'i', 'o', 'u'].iter().cloned().collect();
        fn count(word: &str, k: i32, vowels: &HashSet<char>) -> i64 {
            let n = word.len();
            let mut res = 0;
            let mut consonants = 0;
            let mut occur: HashMap<char, i32> = HashMap::new();
            let mut j = 0;
            let word_chars: Vec<char> = word.chars().collect();
            for i in 0..n {
                while j < n && (consonants < k || occur.len() < 5) {
                    let ch = word_chars[j];
                    if vowels.contains(&ch) {
                        *occur.entry(ch).or_insert(0) += 1;
                    } else {
                        consonants += 1;
                    }
                    j += 1;
                }
                if consonants >= k && occur.len() == 5 {
                    res += (n - j + 1) as i64;
                }
                let left = word_chars[i];
                if vowels.contains(&left) {
                    if let Some(count) = occur.get_mut(&left) {
                        *count -= 1;
                        if *count == 0 {
                            occur.remove(&left);
                        }
                    }
                } else {
                    consonants -= 1;
                }
            }
            res
        }
        count(&word, k, &vowels) - count(&word, k + 1, &vowels)
    }
}

fn main() {
    let tests = vec![("aeioqq", 1, 0), ("aeiou", 0, 1), ("ieaouqqieaouqq", 1, 3)];

    for (word, k, ans) in tests {
        assert_eq!(Solution::count_of_substrings(word.to_string(), k), ans);
    }
}

struct Solution;

impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let mut ans = 1;
        let chars: Vec<char> = word.chars().collect();
        for i in 1..chars.len() {
            if chars[i - 1] == chars[i] {
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![("abbcccc", 5), ("abcd", 1), ("aaaa", 4)];

    for (word, expected) in tests {
        assert_eq!(Solution::possible_string_count(word.to_string()), expected);
    }
}

struct Solution;

impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        s.chars().any(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
    }
}

fn main() {
    let tests = vec![("leetcoder", true), ("bbcd", false)];

    for (s, expected) in tests {
        assert_eq!(Solution::does_alice_win(s.to_string()), expected);
    }
}

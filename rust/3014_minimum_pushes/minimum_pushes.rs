struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let n = word.len() as i32;
        let m = (n - 1) / 8 + 1;
        return (m - 1 + 1) * (m - 1) / 2 * 8 + (n - (m - 1) * 8) * m;
    }
}

fn main() {
    let tests = vec![("abcde", 5), ("xycdefghij", 12)];

    for (word, expected) in tests {
        assert_eq!(Solution::minimum_pushes(word.to_string()), expected);
    }
}

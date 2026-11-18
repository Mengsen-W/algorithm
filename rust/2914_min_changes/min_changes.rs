struct Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        s.as_bytes()
            .chunks_exact(2)
            .filter(|pair| pair[0] != pair[1])
            .count() as i32
    }
}

fn main() {
    let tests = vec![("1001", 2), ("10", 1), ("0000", 0)];

    for (s, expected) in tests {
        assert_eq!(Solution::min_changes(s.to_string()), expected);
    }
}

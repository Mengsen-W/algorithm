struct Solution;

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        seq.chars()
            .enumerate()
            .map(|(i, c)| (i as i32 & 1) ^ if c == '(' { 1 } else { 0 })
            .collect()
    }
}

fn main() {
    let tests = vec![
        ("(()())", vec![0, 1, 1, 1, 1, 0]),
        ("()(())()", vec![0, 0, 0, 1, 1, 0, 1, 1]),
    ];

    for (seq, expected) in tests {
        assert_eq!(Solution::max_depth_after_split(seq.to_string()), expected);
    }
}

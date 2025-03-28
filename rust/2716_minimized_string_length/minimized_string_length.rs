struct Solution;

impl Solution {
    pub fn minimized_string_length(s: String) -> i32 {
        let mut mask = 0u32;
        for c in s.chars() {
            mask |= 1 << (c as u8 - b'a');
        }
        mask.count_ones() as i32
    }
}

fn main() {
    let tests = vec![("aaabc", 3), ("cbbd", 3), ("dddaaa", 2)];

    for (s, expected) in tests {
        assert_eq!(Solution::minimized_string_length(s.to_string()), expected);
    }
}

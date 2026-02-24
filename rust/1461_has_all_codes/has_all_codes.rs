struct Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        use std::collections::HashSet;
        let k = k as usize;
        let total = 1 << k;

        if s.len() < total + k - 1 {
            return false;
        }

        let bytes = s.as_bytes();
        let mut num = 0;
        for i in 0..k {
            num = (num << 1) | (bytes[i] - b'0') as usize;
        }

        let mut exists = HashSet::new();
        exists.insert(num);
        for i in 1..=bytes.len() - k {
            let high_bit = ((bytes[i - 1] - b'0') as usize) << (k - 1);
            num = (num - high_bit) << 1 | (bytes[i + k - 1] - b'0') as usize;
            exists.insert(num);
        }

        exists.len() == total
    }
}

fn main() {
    let tests = vec![("00110110", 2, true), ("0110", 1, true), ("0110", 2, false)];

    for (s, k, expected) in tests {
        assert_eq!(Solution::has_all_codes(s.to_string(), k), expected);
    }
}

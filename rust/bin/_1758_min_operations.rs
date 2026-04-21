struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut cnt = 0;
        let s: Vec<usize> = s.chars().map(|c| c as usize).collect();
        for i in 0..s.len() {
            let c = s[i];
            // 48 == '0'
            if c != (48 + i % 2) {
                cnt += 1;
            }
        }
        cnt.min(s.len() as i32 - cnt)
    }
}

fn main() {
    let tests = vec![("0100", 1), ("10", 0), ("1111", 2)];

    for (s, expected) in tests {
        assert_eq!(Solution::min_operations(s.to_string()), expected);
    }
}

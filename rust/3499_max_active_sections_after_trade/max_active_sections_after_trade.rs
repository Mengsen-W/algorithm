struct Solution;

impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        let n = s.len();
        let cnt1 = s.chars().filter(|&c| c == '1').count() as i32;

        let mut i = 0;
        let mut best_gain = 0;
        let mut prev = i32::MIN;
        let mut cur;
        let bytes = s.as_bytes();

        while i < n {
            let start = i;
            while i < n && bytes[i] == bytes[start] {
                i += 1;
            }
            if bytes[start] == b'0' {
                cur = (i - start) as i32;
                if prev != i32::MIN {
                    best_gain = best_gain.max(prev + cur);
                }
                prev = cur;
            }
        }

        cnt1 + best_gain
    }
}

fn main() {
    let tests = vec![("01", 1), ("0100", 4), ("1000100", 7), ("01010", 4)];

    for (s, ans) in tests {
        assert_eq!(
            Solution::max_active_sections_after_trade(s.to_string()),
            ans
        );
    }
}

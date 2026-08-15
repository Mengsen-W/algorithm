struct Solution;

impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut count = [0usize; 26];
        let mut left = 0usize;
        let mut res = 0usize;

        for right in 0..bytes.len() {
            let ch = (bytes[right] - b'a') as usize;
            count[ch] += 1;

            while count[ch] > 2 {
                let ch2 = (bytes[left] - b'a') as usize;
                count[ch2] -= 1;
                left += 1;
            }

            res = res.max(right - left + 1);
        }

        res as i32
    }
}

fn main() {
    let tests = vec![("bcbbbcba", 4), ("aaaa", 2)];

    for (s, ans) in tests {
        assert_eq!(Solution::maximum_length_substring(s.to_string()), ans);
    }
}

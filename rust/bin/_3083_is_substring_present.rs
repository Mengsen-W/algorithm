struct Solution;

impl Solution {
    pub fn is_substring_present(s: String) -> bool {
        let mut h = vec![0; 26];
        let s = s.as_bytes();
        for i in 0..s.len() - 1 {
            let x = (s[i] - b'a') as usize;
            let y = (s[i + 1] - b'a') as usize;
            h[x] |= 1 << y;
            if h[y] >> x & 1 == 1 {
                return true;
            }
        }
        false
    }
}

fn main() {
    let tests = vec![("leetcode", true), ("abcba", true), ("abcd", false)];

    for (s, ans) in tests {
        assert_eq!(Solution::is_substring_present(s.to_string()), ans);
    }
}

struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut ans = 0;
        let mut cnt = [0; 3];

        let mut l = 0;
        let mut r = 0;

        while l < n {
            while r < n && (cnt[0] == 0 || cnt[1] == 0 || cnt[2] == 0) {
                cnt[(s[r] - b'a') as usize] += 1;
                r += 1;
            }

            if cnt[0] > 0 && cnt[1] > 0 && cnt[2] > 0 {
                ans += (n - r + 1) as i32;
            }

            cnt[(s[l] - b'a') as usize] -= 1;
            l += 1;
        }

        ans
    }
}

fn main() {
    let tests = vec![("abcabc", 10), ("aaacb", 3), ("abc", 1)];

    for (s, expected) in tests {
        assert_eq!(Solution::number_of_substrings(s.to_string()), expected);
    }
}

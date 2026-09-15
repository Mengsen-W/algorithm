struct Solution;

impl Solution {
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        fn check(s: &[u8], mut l: usize, mut r: usize) -> bool {
            while l < r {
                if s[l] != s[r] {
                    return false;
                }
                l += 1;
                r -= 1;
            }
            true
        }

        let s = s.as_bytes();
        let n = s.len();
        let k = k as usize;
        let mut ans = 0;
        let mut start = 0;

        for r in k - 1..n {
            let mut l = r + 1 - k;
            if l >= start && check(s, l, r) {
                ans += 1;
                start = r + 1;
                continue;
            }

            if r >= k {
                l = r - k;
                if l >= start && check(s, l, r) {
                    ans += 1;
                    start = r + 1;
                }
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![("abac{dbbd", 3, 2), ("adbcda", 2, 0)];

    for (s, k, ans) in tests {
        assert_eq!(Solution::max_palindromes(s.to_string(), k), ans);
    }
}

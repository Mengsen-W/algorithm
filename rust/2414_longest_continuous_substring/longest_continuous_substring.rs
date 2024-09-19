struct Solution;

impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let mut res = 1;
        let mut cur = 1;
        for i in 1..s.len() {
            if s.as_bytes()[i] == s.as_bytes()[i - 1] + 1 {
                cur += 1;
            } else {
                cur = 1;
            }
            res = i32::max(res, cur);
        }
        res
    }
}

fn main() {
    let tests = vec![("abacaba", 2), ("abcde", 5)];

    for (s, ans) in tests {
        assert_eq!(Solution::longest_continuous_substring(s.to_string()), ans);
    }
}

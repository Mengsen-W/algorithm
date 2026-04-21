struct Solution;

impl Solution {
    pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
        let mut res = 0;
        let mut cnt1 = 0;
        let mut cnt2 = 0;
        for c in text.chars() {
            if c == pattern.chars().nth(1).unwrap() {
                res += cnt1;
                cnt2 += 1;
            }
            if c == pattern.chars().nth(0).unwrap() {
                cnt1 += 1;
            }
        }
        res + std::cmp::max(cnt1, cnt2)
    }
}

fn main() {
    let tests = vec![("abdcdbc", "ac", 4), ("aabb", "ab", 6)];

    for (text, pattern, ans) in tests {
        assert_eq!(
            Solution::maximum_subsequence_count(text.to_string(), pattern.to_string()),
            ans
        );
    }
}

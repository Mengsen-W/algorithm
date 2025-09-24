struct Solution;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let k = k as usize;
        let mut res = s.clone();
        let mut left: usize = 0;
        let mut right: usize;
        for _ in (0..s.len()).step_by(2 * k) {
            right = s.len().min(left + k);
            let word = s[left..right].chars().rev().collect::<String>();
            res.replace_range(left..right, word.as_str());
            left += 2 * k;
        }
        res
    }
}

fn main() {
    let tests = vec![("abcdefg", 2, "bacdfeg"), ("abcd", 2, "bacd")];

    for (s, k, ans) in tests {
        assert_eq!(Solution::reverse_str(s.to_string(), k), ans.to_string());
    }
}

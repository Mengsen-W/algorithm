struct Solution;

impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        let mut ans = 0;
        let chars: Vec<char> = s.chars().collect();
        for i in 1..chars.len() {
            if chars[i - 1].to_lowercase().to_string() != chars[i].to_lowercase().to_string() {
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![("aAbBcC", 2), ("AaAaAaaA", 0)];

    for (s, ans) in tests {
        assert_eq!(Solution::count_key_changes(s.to_string()), ans);
    }
}

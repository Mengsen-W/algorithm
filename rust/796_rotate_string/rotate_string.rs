struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        s.len() == goal.len() && goal.repeat(2).contains(&s)
    }
}

fn main() {
    assert_eq!(
        Solution::rotate_string("abcde".to_string(), "cdeab".to_string()),
        true
    );
    assert_eq!(
        Solution::rotate_string("abcde".to_string(), "abced".to_string()),
        false
    );
}

struct Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let (mut bal, mut res) = (0, 0);
        for i in 0..s.len() {
            bal += if s[i] == '(' { 1 } else { -1 };
            if s[i] == ')' && s[i - 1] == '(' {
                res += 1 << bal;
            }
        }
        res
    }
}

fn main() {
    let tests = vec![("()", 1), ("(())", 2), ("()()", 2), ("(()(()))", 6)];

    for (s, expected) in tests {
        assert_eq!(Solution::score_of_parentheses(s.to_string()), expected);
    }
}

struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        s.chars()
            .fold((String::new(), 0), |(mut acc, mut level), c| {
                if c == ')' {
                    level -= 1;
                }
                if level != 0 {
                    acc.push(c);
                }
                if c == '(' {
                    level += 1;
                }
                (acc, level)
            })
            .0
    }
}

fn main() {
    let tests = vec![
        ("(()())(())", "()()()"),
        ("(()())(())(()(()))", "()()()()(())"),
        ("()()", ""),
    ];

    for (s, expected) in tests {
        assert_eq!(Solution::remove_outer_parentheses(s.to_string()), expected);
    }
}

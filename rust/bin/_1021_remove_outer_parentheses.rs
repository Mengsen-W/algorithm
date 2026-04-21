/*
 * @Date: 2022-05-28 10:30:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-28 10:40:21
 * @FilePath: /algorithm/1021_remove_outer_parentheses/remove_outer_parentheses.rs
 */

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

fn main() {
    assert_eq!(
        remove_outer_parentheses(String::from("(()())(())")),
        String::from("()()()")
    );
    assert_eq!(
        remove_outer_parentheses(String::from("(()())(())(()(()))")),
        String::from("()()()()(())")
    );
    assert_eq!(
        remove_outer_parentheses(String::from("()()")),
        String::new()
    );
}

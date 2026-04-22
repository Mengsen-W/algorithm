/*
 * @Date: 2022-10-09
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-09
 * @FilePath: /algorithm/856_score_of_parentheses/score_of_parentheses.rs
 */

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

fn main() {
    assert_eq!(score_of_parentheses(String::from("()")), 1);
    assert_eq!(score_of_parentheses(String::from("(())")), 2);
    assert_eq!(score_of_parentheses(String::from("()()")), 2);
    assert_eq!(score_of_parentheses(String::from("(()(()))")), 6);
}

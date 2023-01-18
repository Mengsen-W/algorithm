/*
 * @Date: 2023-01-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-19
 * @FilePath: /algorithm/rust/src/2299_strong_password_checker_ii/strong_password_checker_ii.rs
 */

pub fn strong_password_checker_ii(password: String) -> bool {
    let n = password.len();
    if n < 8 {
        return false;
    }
    let (mut lowercase, mut uppercase) = (0, 0);
    let (mut digit, mut special) = (0, 0);
    let mut prev = '🐕';
    for ch in password.chars() {
        match ch {
            'a'..='z' => lowercase += 1,
            'A'..='Z' => uppercase += 1,
            '0'..='9' => digit += 1,
            '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')' | '-' | '+' => special += 1,
            _ => (),
        }
        if ch == prev {
            return false;
        }
        prev = ch;
    }
    lowercase > 0 && uppercase > 0 && digit > 0 && special > 0
}

fn main() {
    assert!(strong_password_checker_ii(String::from("IloveLe3tcode!")));
    assert!(!strong_password_checker_ii(String::from(
        "Me+You--IsMyDream"
    )));
    assert!(!strong_password_checker_ii(String::from("1aB!")));
}

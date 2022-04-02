/*
 * @Date: 2022-04-02 00:39:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-02 01:06:26
 * @FilePath: /algorithm/420_strong_password_checker/strong_password_checker.rs
 */

pub fn strong_password_checker(password: String) -> i32 {
    let password = password.chars().collect::<Vec<char>>();
    let n = password.len();
    let (mut has_lower, mut has_upper, mut has_dight) = (false, false, false);

    for ch in &password {
        if ch.is_lowercase() {
            has_lower = true;
        } else if ch.is_uppercase() {
            has_upper = true;
        } else if ch.is_digit(10) {
            has_dight = true;
        }
    }

    let category = if has_lower { 1 } else { 0 }
        + if has_upper { 1 } else { 0 }
        + if has_dight { 1 } else { 0 };

    if n < 6 {
        return (6 - n).max(3 - category) as i32;
    } else if n <= 20 {
        let mut replace = 0;
        let mut cnt = 0;
        let mut cur = '#';

        for ch in &password {
            if *ch == cur {
                cnt += 1;
            } else {
                replace += cnt / 3;
                cnt = 1;
                cur = *ch;
            }
        }
        replace += cnt / 3;
        return replace.max(3 - category) as i32;
    } else {
        let (mut replace, mut remove) = (0, n - 20);
        let (mut rm2, mut cnt, mut cur) = (0, 0, '#');

        for ch in &password {
            if *ch == cur {
                cnt += 1;
            } else {
                if remove > 0 && cnt >= 3 {
                    if cnt % 3 == 0 {
                        remove -= 1;
                        replace -= 1;
                    } else if cnt % 3 == 1 {
                        rm2 += 1;
                    }
                }

                replace += cnt / 3;
                cnt = 1;
                cur = *ch;
            }
        }

        if remove > 0 && cnt >= 3 {
            if cnt % 3 == 0 {
                remove -= 1;
                replace -= 1;
            } else if cnt % 3 == 1 {
                rm2 += 1
            }
        }

        replace += cnt / 3;

        let use2 = replace.min(rm2).min(remove / 2);
        replace -= use2;
        remove -= use2 * 2;
        let use3 = replace.min(remove / 3);
        replace -= use3;
        // remove -= use3 * 3;
        return (n - 20) as i32 + replace.max(3 - category) as i32;
    }
}

fn main() {
    assert_eq!(strong_password_checker(String::from("a")), 5);
    assert_eq!(strong_password_checker(String::from("aA1")), 3);
    assert_eq!(strong_password_checker(String::from("1337C0d3")), 0);
}

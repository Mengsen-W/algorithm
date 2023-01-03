/*
 * @Date: 2023-01-03
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2023-01-03
 * @FilePath: /algorithm/2024_are_numbers_ascending/are_numbers_ascending.rs
 */

pub fn are_numbers_ascending(s: String) -> bool {
    let s = s.into_bytes();
    let (mut pre, mut pos) = (0, 0);
    while pos < s.len() {
        if s[pos].is_ascii_digit() {
            let mut cur = 0;
            while pos < s.len() && s[pos].is_ascii_digit() {
                cur = cur * 10 + s[pos] - 48;
                pos += 1;
            }
            if cur <= pre {
                return false;
            }
            pre = cur;
        } else {
            pos += 1;
        }
    }
    return true;
}

fn main() {
    {
        let s = String::from("1 box has 3 blue 4 red 6 green and 12 yellow marbles");
        let ans = true;
        assert_eq!(are_numbers_ascending(s), ans);
    }

    {
        let s = String::from("hello world 5 x 5");
        let ans = false;
        assert_eq!(are_numbers_ascending(s), ans);
    }

    {
        let s = String::from("sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s");
        let ans = false;
        assert_eq!(are_numbers_ascending(s), ans);
    }

    {
        let s = String::from("4 5 11 26");
        let ans = true;
        assert_eq!(are_numbers_ascending(s), ans);
    }
}

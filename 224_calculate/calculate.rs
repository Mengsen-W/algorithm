/*
 * @Date: 2021-03-10 08:10:16
 * @Author: mengsen
 * @LastEditors: mengsen
 * @LastEditTime: 2021-03-10 08:18:48
 */

fn calculate(s: String) -> i32 {
    //let s = s.replace(' ', "");
    rec(s.as_bytes()).0
}

fn rec(s: &[u8]) -> (i32, usize) {
    let mut acc = 0;
    let mut add = true;
    let mut i = 0;

    while i < s.len() {
        match s[i] {
            b'+' => add = true,
            b'-' => add = false,
            b'(' => {
                let (v, len) = rec(&s[i + 1..]);
                if add {
                    acc += v;
                } else {
                    acc -= v;
                }
                i += len;
            }
            b')' => return (acc, i + 1),
            b' ' => {}
            n => {
                // must be num!
                assert!(matches!(n, b'0'..=b'9'));
                let mut n = (n - b'0') as i32;
                i += 1;
                while i < s.len() && matches!(s[i], b'0'..=b'9') {
                    n = n * 10 + (s[i] - b'0') as i32;
                    i += 1;
                }
                if i < s.len() {
                    i -= 1;
                }
                if add {
                    acc += n;
                } else {
                    acc -= n;
                }
            }
        }
        i += 1;
    }
    (acc, i)
}

fn main() {
    assert_eq!(calculate("1 + 1".to_string()), 2);
    assert_eq!(calculate("2 - 1 + 2".to_string()), 3);
    assert_eq!(calculate("(1 + (4 + 5 + 2) - 3) + (6 + 8)".to_string()), 23);
}

/*
 * @Date: 2021-03-11 08:17:15
 * @Author: mengsen
 * @LastEditors: mengsen
 * @LastEditTime: 2021-03-11 08:24:16
 */

fn calculate(s: String) -> i32 {
    let mut st = Vec::new();
    let mut last_op = b'+';
    let mut n = 0;

    for (i, b) in s.bytes().enumerate() {
        let is_digit = b.is_ascii_digit();

        if is_digit {
            n = n * 10 + (b - b'0') as i32;
        }

        if !is_digit && b != b' ' || i == s.len() - 1 {
            match last_op {
                b'+' => st.push(n),
                b'-' => st.push(-n),
                b'*' => *st.last_mut().unwrap() *= n,
                b'/' => *st.last_mut().unwrap() /= n,
                _ => unreachable!(),
            }

            last_op = b;
            n = 0;
        }
    }

    st.iter().sum()
}

fn main() {
    assert_eq!(calculate(String::from("3+2*2")), 7);
    assert_eq!(calculate(String::from("3/2")), 1);
    assert_eq!(calculate(String::from("3+5 / 2")), 5);
}

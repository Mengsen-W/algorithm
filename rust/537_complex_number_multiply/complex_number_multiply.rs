/*
 * @Date: 2022-02-25 00:25:48
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-25 00:42:25
 * @FilePath: /algorithm/537_complex_number_multiply/complex_number_multiply.rs
 */

pub fn complex_number_multiply(num1: String, num2: String) -> String {
    fn str_to_complex(s: String) -> (i32, i32) {
        let idx = s.find('+').unwrap();
        (
            (&s[0..idx]).parse().unwrap(),
            (&s[idx + 1..s.len() - 1]).parse().unwrap(),
        )
    }
    let x = str_to_complex(num1);
    let y = str_to_complex(num2);
    format!("{}+{}i", x.0 * y.0 - x.1 * y.1, x.0 * y.1 + x.1 * y.0)
}

fn main() {
    assert_eq!(
        complex_number_multiply("1+1i".to_string(), "1+1i".to_string()),
        "0+2i"
    );

    assert_eq!(
        complex_number_multiply("1+-1i".to_string(), "1+-1i".to_string()),
        "0+-2i"
    );
}
